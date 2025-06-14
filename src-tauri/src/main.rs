#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use tauri::{Manager, PhysicalPosition, PhysicalSize, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use windows::Win32::Foundation::HWND;

fn toggle_click_through(hwnd: HWND, enable: bool) {
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT,
    };

    unsafe {
        let style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        let mut new_style = style | WS_EX_LAYERED.0 as i32;

        if enable {
            new_style |= WS_EX_TRANSPARENT.0 as i32;
        } else {
            new_style &= !(WS_EX_TRANSPARENT.0 as i32);
        }

        SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);
    }
}

fn snap_window_to_right(
    window: &WebviewWindow,
    monitor_pos: PhysicalPosition<i32>,
    monitor_size: PhysicalSize<u32>,
) {
    if let Ok(window_size) = window.outer_size() {
        let new_x = monitor_pos.x + (monitor_size.width as i32) - (window_size.width as i32);
        let new_y = monitor_pos.y;
        let _ = window.set_position(PhysicalPosition::new(new_x, new_y));
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            let (initial_monitor_pos, initial_monitor_size) = match window.current_monitor().unwrap() {
                Some(monitor) => (monitor.position().clone(), monitor.size().clone()),
                None => (PhysicalPosition::new(0, 0), PhysicalSize::new(1920, 1080)),
            };

            snap_window_to_right(&window, initial_monitor_pos, initial_monitor_size);

            #[cfg(desktop)]
            {
                let interactive = Arc::new(Mutex::new(true));
                let collapsed = Arc::new(Mutex::new(false));
                let monitor_pos = Arc::new(Mutex::new(initial_monitor_pos));
                let monitor_size = Arc::new(Mutex::new(initial_monitor_size));
                let window_label = "main".to_string();

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["ctrl+shift+alt+o"])?
                        .with_handler({
                            let interactive = interactive.clone();
                            let collapsed = collapsed.clone();
                            let monitor_pos = monitor_pos.clone();
                            let monitor_size = monitor_size.clone();

                            move |app_handle, shortcut, event| {
                                if event.state == ShortcutState::Pressed
                                    && shortcut.matches(
                                        Modifiers::CONTROL | Modifiers::SHIFT | Modifiers::ALT,
                                        Code::KeyO,
                                    )
                                {
                                    let mut is_interactive = interactive.lock().unwrap();
                                    let mut is_collapsed = collapsed.lock().unwrap();

                                    *is_interactive = !*is_interactive;
                                    *is_collapsed = !*is_collapsed;

                                    if let Some(window) = app_handle.get_webview_window(&window_label) {
                                        if let Ok(hwnd) = window.hwnd() {
                                            toggle_click_through(HWND(hwnd.0), !*is_interactive);
                                        }

                                        if *is_collapsed {
                                            if let Ok(size) = window.outer_size() {
                                                let _ = window.set_size(PhysicalSize::new(0, size.height));
                                            }
                                        } else {
                                            let _ = window.set_size(PhysicalSize::new(320, 1000));
                                        }

                                        snap_window_to_right(
                                            &window,
                                            *monitor_pos.lock().unwrap(),
                                            *monitor_size.lock().unwrap(),
                                        );

                                        println!(
                                            "Window is now {} and {}",
                                            if *is_interactive {
                                                "interactive"
                                            } else {
                                                "click-through"
                                            },
                                            if *is_collapsed {
                                                "collapsed"
                                            } else {
                                                "expanded"
                                            }
                                        );
                                    }
                                }
                            }
                        })
                        .build(),
                )?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
