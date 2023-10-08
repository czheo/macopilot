// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayEvent, Menu, SystemTrayMenu, Submenu, PhysicalPosition};
use tauri::{WindowBuilder, WindowEvent};
use tauri::Manager;

const INIT_WINDOW_WIDTH: f64 = 450.0;
const INIT_WINDOW_HEIGHT: f64 = 600.0;
const MAIN_WIN_LABEL: &str = "main-win";

fn toggle_window_visibility(app: &tauri::AppHandle, position: PhysicalPosition<f64>) {
    match app.get_window(MAIN_WIN_LABEL) {
        Some(window) => {
            match window.is_visible() {
                Ok(true) => {
                    let _ = window.hide();
                }
                _ => {
                    let win_size = window.inner_size().unwrap();
                    let _ = window.set_position(PhysicalPosition::new(
                            position.x - f64::from(win_size.width) / 2.0,
                            position.y));
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
        None => {
            let _window = WindowBuilder::new(
                app,
                MAIN_WIN_LABEL.to_string(),
                tauri::WindowUrl::External("https://chat.openai.com/".parse().unwrap())
            )
                .title("Macopilot")
                .inner_size(INIT_WINDOW_WIDTH, INIT_WINDOW_HEIGHT)
                .position(position.x - INIT_WINDOW_WIDTH / 2.0, position.y)
                .closable(false)
                .build()
                .unwrap();
        }
    }
}

fn main() {
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray = SystemTray::new();
        // .with_menu(SystemTrayMenu::new().add_item(quit));
    let menu = Menu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    tauri::Builder::default()
        .menu(menu)
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position,
                size: _,
                ..
            } => {
                toggle_window_visibility(app, position);
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::Focused(false) => {
                if event.window().label() == MAIN_WIN_LABEL {
                    let _ = event.window().hide();
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
