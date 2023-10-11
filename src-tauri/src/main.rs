// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayEvent, Menu, SystemTrayMenu, MenuItem, Submenu, AboutMetadata};
use tauri::{WindowBuilder, WindowEvent};
use tauri::Manager;
use tauri::GlobalShortcutManager;
// use tauri::ActivationPolicy;

const APP_NAME: &str = "Macopilot";
const MAIN_WIN_LABEL: &str = "main-win";
const QUITE_MENU_ITEM_LABEL: &str = "quit";
const SETTINGS_MENU_ITEM_LABEL: &str = "settings";
const CHAT_GPT_MENU_ITEM_LABEL: &str = "chat-gpt";
const KEYBOARD_SHOTCUT: &str = "CTRL + G";

fn init_window(app: &tauri::AppHandle) {
    let _window = WindowBuilder::new(
        app,
        MAIN_WIN_LABEL.to_string(),
        tauri::WindowUrl::External("https://chat.openai.com/".parse().unwrap())
    )
        .title(APP_NAME)
        .build()
        .unwrap();
}

fn show_window(app: &tauri::AppHandle) {
    match app.get_window(MAIN_WIN_LABEL) {
        Some(window) => {
            let _ = window.show();
            let _ = window.set_focus();
        }
        _ => {}
    }
}

fn toggle_window(app: &tauri::AppHandle) {
    match app.get_window(MAIN_WIN_LABEL) {
        Some(window) => {
            do_toggle_window(&window);
        }
        _ => {}
    }
}

fn do_toggle_window(window: &tauri::Window) {
    if let Ok(true) = window.is_visible() {
        let _ = window.hide();
    } else {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn register_hot_key(app_handle: tauri::AppHandle) {
    let mut gsm = app_handle.global_shortcut_manager();
    let _ = gsm.register(KEYBOARD_SHOTCUT, move || {
        toggle_window(&app_handle);
    });
}

fn create_system_tray() -> SystemTray {
    let chat_gpt = CustomMenuItem::new(CHAT_GPT_MENU_ITEM_LABEL, "ChatGPT")
        .accelerator(KEYBOARD_SHOTCUT);
    // TODO: making shotcut configurable in settings
    let _settings = CustomMenuItem::new(SETTINGS_MENU_ITEM_LABEL, "Settings");
    let quit = CustomMenuItem::new(QUITE_MENU_ITEM_LABEL, "Quit");

    return SystemTray::new()
        .with_menu(SystemTrayMenu::new()
            .add_item(chat_gpt)
            // .add_item(settings)
            .add_item(quit));
}

fn create_menu() -> Menu {
    let app_menu = Submenu::new(
        APP_NAME,
        Menu::new()
        .add_native_item(
            MenuItem::About(
                APP_NAME.to_string(),
                AboutMetadata::new()
            )
        )
    );
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::SelectAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
    );
    return Menu::new()
        .add_submenu(app_menu)
        .add_submenu(edit_menu);
}

fn on_setup(app: &mut tauri::App) {
    // app.set_activation_policy(ActivationPolicy::Accessory);
    let app_handle = app.handle();
    init_window(&app_handle);
    register_hot_key(app_handle);
}


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            on_setup(app);
            Ok(())
        })
        .menu(create_menu())
        .system_tray(create_system_tray())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    CHAT_GPT_MENU_ITEM_LABEL => {
                        show_window(app);
                    }
                    QUITE_MENU_ITEM_LABEL => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested{api, ..} => {
                if event.window().label() == MAIN_WIN_LABEL {
                    api.prevent_close();
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
