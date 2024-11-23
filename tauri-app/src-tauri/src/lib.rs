// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::async_runtime::spawn;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tokio::time::{sleep, Duration};

// This is an example of an async function that can be called from the webview
async fn emit_event(app: AppHandle) -> Result<(), String> {
    let mut i = 0;
    loop {
        let payload = format!("some time has passed {}", i);
        app.emit("my_event", payload).map_err(|e| e.to_string())?;
        i += 1;
        sleep(Duration::from_secs(5)).await;
    }
}

// Expose a command that the webview can call
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn setup_global_shortcuts<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts(["ctrl+d", "shift+k", "cmd+shift+k"])
        .unwrap()
        .with_handler(|app, shortcut, event| {
            if event.state == ShortcutState::Pressed {
                let message = match (shortcut.key, shortcut.mods) {
                    (Code::KeyD, mods) if mods == Modifiers::CONTROL => "Ctrl+D triggered",
                    (Code::KeyK, mods) if mods == (Modifiers::SUPER | Modifiers::SHIFT) => {
                        "Cmd+Shift+K triggered"
                    }
                    (Code::KeyK, mods) if mods == Modifiers::SHIFT => "Shift+K triggered",
                    _ => return,
                };
                let _ = app.emit("shortcut-event", message);
            }
        })
        .build()
}

// The main function that runs the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(setup_global_shortcuts())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Spawn setup as a non-blocking task so the windows can be
            // created and ran while it executes
            spawn(emit_event(app.handle().clone()));
            // The hook expects an Ok result

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    other => {
                        println!("menu item {} not handled", other);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
