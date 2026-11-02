use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaSessionInfo {
    pub title: String,
    pub artist: String,
    pub is_playing: bool,
    pub has_media: bool,
}

#[cfg(windows)]
#[tauri::command]
fn get_media_info() -> Result<MediaSessionInfo, String> {
    use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

    let op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .map_err(|e| e.to_string())?;
    let session_manager = op.get().map_err(|e| e.to_string())?;

    if let Ok(current_session) = session_manager.GetCurrentSession() {
        let is_playing = if let Ok(playback_info) = current_session.GetPlaybackInfo() {
            if let Ok(status) = playback_info.PlaybackStatus() {
                status == windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
            } else {
                false
            }
        } else {
            false
        };

        if let Ok(media_properties_op) = current_session.TryGetMediaPropertiesAsync() {
            if let Ok(props) = media_properties_op.get() {
                let title = props.Title().map(|s| s.to_string()).unwrap_or_default();
                let artist = props.Artist().map(|s| s.to_string()).unwrap_or_default();

                return Ok(MediaSessionInfo {
                    title: if title.is_empty() { "Desconhecido".to_string() } else { title },
                    artist: if artist.is_empty() { "Artista Desconhecido".to_string() } else { artist },
                    is_playing,
                    has_media: true,
                });
            }
        }
    }

    Ok(MediaSessionInfo {
        title: "Nenhuma mídia".to_string(),
        artist: "Windows Media".to_string(),
        is_playing: false,
        has_media: false,
    })
}

#[cfg(not(windows))]
#[tauri::command]
fn get_media_info() -> Result<MediaSessionInfo, String> {
    Ok(MediaSessionInfo {
        title: "Mock Title".to_string(),
        artist: "Mock Artist".to_string(),
        is_playing: false,
        has_media: false,
    })
}

#[cfg(windows)]
#[tauri::command]
fn toggle_play_pause() -> Result<bool, String> {
    use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

    let op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .map_err(|e| e.to_string())?;
    let session_manager = op.get().map_err(|e| e.to_string())?;

    if let Ok(current_session) = session_manager.GetCurrentSession() {
        let _ = current_session.TryTogglePlayPauseAsync();
        return Ok(true);
    }
    Ok(false)
}

#[cfg(not(windows))]
#[tauri::command]
fn toggle_play_pause() -> Result<bool, String> {
    Ok(false)
}

#[cfg(windows)]
#[tauri::command]
fn skip_next() -> Result<bool, String> {
    use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

    let op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .map_err(|e| e.to_string())?;
    let session_manager = op.get().map_err(|e| e.to_string())?;

    if let Ok(current_session) = session_manager.GetCurrentSession() {
        let _ = current_session.TrySkipNextAsync();
        return Ok(true);
    }
    Ok(false)
}

#[cfg(not(windows))]
#[tauri::command]
fn skip_next() -> Result<bool, String> {
    Ok(false)
}

#[cfg(windows)]
#[tauri::command]
fn skip_previous() -> Result<bool, String> {
    use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

    let op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .map_err(|e| e.to_string())?;
    let session_manager = op.get().map_err(|e| e.to_string())?;

    if let Ok(current_session) = session_manager.GetCurrentSession() {
        let _ = current_session.TrySkipPreviousAsync();
        return Ok(true);
    }
    Ok(false)
}

#[cfg(not(windows))]
#[tauri::command]
fn skip_previous() -> Result<bool, String> {
    Ok(false)
}

#[cfg(windows)]
fn setup_window_overlay(window: &tauri::WebviewWindow) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SetWindowPos, GWL_EXSTYLE, SM_CXSCREEN, SWP_NOACTIVATE, SWP_SHOWWINDOW,
        WS_EX_TOOLWINDOW, HWND_TOPMOST,
    };

    if let Ok(hwnd_ptr) = window.hwnd() {
        let hwnd = HWND(hwnd_ptr.0 as _);
        unsafe {
            // Remove from Alt+Tab
            let ex_style = windows::Win32::UI::WindowsAndMessaging::GetWindowLongW(hwnd, GWL_EXSTYLE);
            windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(
                hwnd,
                GWL_EXSTYLE,
                ex_style | WS_EX_TOOLWINDOW.0 as i32,
            );

            // Center horizontally at the top of the monitor
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let win_width = 600;
            let win_height = 280;
            let pos_x = (screen_width - win_width) / 2;
            let pos_y = 12;

            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                pos_x,
                pos_y,
                win_width,
                win_height,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_media_info,
            toggle_play_pause,
            skip_next,
            skip_previous
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(windows)]
                setup_window_overlay(&window);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
