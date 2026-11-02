use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaSessionInfo {
    pub title: String,
    pub artist: String,
    pub is_playing: bool,
    pub has_media: bool,
    pub thumbnail_base64: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemStatusInfo {
    pub battery_percent: u8,
    pub is_charging: bool,
    pub has_battery: bool,
}

#[cfg(windows)]
#[tauri::command]
async fn get_media_info() -> Result<MediaSessionInfo, String> {
    tauri::async_runtime::spawn_blocking(|| {
        use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

        if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
            if let Ok(session_manager) = op.get() {
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
                                thumbnail_base64: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(MediaSessionInfo {
            title: "Nenhuma mídia".to_string(),
            artist: "Windows Media".to_string(),
            is_playing: false,
            has_media: false,
            thumbnail_base64: None,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(not(windows))]
#[tauri::command]
async fn get_media_info() -> Result<MediaSessionInfo, String> {
    Ok(MediaSessionInfo {
        title: "Mock Title".to_string(),
        artist: "Mock Artist".to_string(),
        is_playing: false,
        has_media: false,
        thumbnail_base64: None,
    })
}

#[cfg(windows)]
#[tauri::command]
fn get_system_status() -> Result<SystemStatusInfo, String> {
    use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

    let mut status = SYSTEM_POWER_STATUS::default();
    unsafe {
        if GetSystemPowerStatus(&mut status).is_ok() {
            let has_battery = status.BatteryFlag != 128 && status.BatteryLifePercent != 255;
            let battery_percent = if status.BatteryLifePercent <= 100 {
                status.BatteryLifePercent
            } else {
                100
            };
            let is_charging = status.ACLineStatus == 1;

            return Ok(SystemStatusInfo {
                battery_percent,
                is_charging,
                has_battery,
            });
        }
    }

    Ok(SystemStatusInfo {
        battery_percent: 100,
        is_charging: true,
        has_battery: false,
    })
}

#[cfg(not(windows))]
#[tauri::command]
fn get_system_status() -> Result<SystemStatusInfo, String> {
    Ok(SystemStatusInfo {
        battery_percent: 100,
        is_charging: true,
        has_battery: false,
    })
}

#[cfg(windows)]
#[tauri::command]
async fn toggle_play_pause() -> Result<bool, String> {
    tauri::async_runtime::spawn_blocking(|| {
        use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

        if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
            if let Ok(session_manager) = op.get() {
                if let Ok(current_session) = session_manager.GetCurrentSession() {
                    let _ = current_session.TryTogglePlayPauseAsync();
                    return Ok(true);
                }
            }
        }
        Ok(false)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(not(windows))]
#[tauri::command]
async fn toggle_play_pause() -> Result<bool, String> {
    Ok(false)
}

#[cfg(windows)]
#[tauri::command]
async fn skip_next() -> Result<bool, String> {
    tauri::async_runtime::spawn_blocking(|| {
        use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

        if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
            if let Ok(session_manager) = op.get() {
                if let Ok(current_session) = session_manager.GetCurrentSession() {
                    let _ = current_session.TrySkipNextAsync();
                    return Ok(true);
                }
            }
        }
        Ok(false)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(not(windows))]
#[tauri::command]
async fn skip_next() -> Result<bool, String> {
    Ok(false)
}

#[cfg(windows)]
#[tauri::command]
async fn skip_previous() -> Result<bool, String> {
    tauri::async_runtime::spawn_blocking(|| {
        use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

        if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
            if let Ok(session_manager) = op.get() {
                if let Ok(current_session) = session_manager.GetCurrentSession() {
                    let _ = current_session.TrySkipPreviousAsync();
                    return Ok(true);
                }
            }
        }
        Ok(false)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(not(windows))]
#[tauri::command]
async fn skip_previous() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
fn set_ignore_cursor_events(app: tauri::AppHandle, ignore: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_ignore_cursor_events(ignore);
    }
    Ok(())
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
            let ex_style = windows::Win32::UI::WindowsAndMessaging::GetWindowLongW(hwnd, GWL_EXSTYLE);
            windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(
                hwnd,
                GWL_EXSTYLE,
                ex_style | WS_EX_TOOLWINDOW.0 as i32,
            );

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
            get_system_status,
            toggle_play_pause,
            skip_next,
            skip_previous,
            set_ignore_cursor_events
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
