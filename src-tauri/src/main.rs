// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::new()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app: &mut App| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                println!("Initializing...");
                let target = "https://huggingface.co/chat";
                let eval_string = format!("window.location.replace('{}')", target);
                match main_window.eval(&eval_string) {
                    Ok(_) => println!("Evaluated JS successfully"),
                    Err(e) => println!("Error evaluating JS: {}", e),
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Done initializing.");

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
