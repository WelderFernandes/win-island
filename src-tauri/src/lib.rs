use tauri::Manager;

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
            // Remove from Alt+Tab switcher by setting WS_EX_TOOLWINDOW
            let ex_style = windows::Win32::UI::WindowsAndMessaging::GetWindowLongW(hwnd, GWL_EXSTYLE);
            windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(
                hwnd,
                GWL_EXSTYLE,
                ex_style | WS_EX_TOOLWINDOW.0 as i32,
            );

            // Center window horizontally at the top of the primary monitor
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let win_width = 600;
            let win_height = 280;
            let pos_x = (screen_width - win_width) / 2;
            let pos_y = 12; // Small gap from the top bezel

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
