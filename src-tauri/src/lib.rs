mod weather;

use std::{thread, time::Duration};
use tauri::{Emitter, Manager};

use weather::fetch_weather;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
          start_weather_worker(app.app_handle().clone());
          Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, fetch_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn start_weather_worker(app_handle: tauri::AppHandle) {
  thread::spawn(move || {
      loop {
        let result = fetch_weather(
          "Irkutsk".into(),
          "ru".into(),
          "4b7f29a8e15af3ec8d463f83ce5dd419".into(),
        );
          if let Ok(data) = result {
              let _ = app_handle.emit("weather_update", data);
          }
          thread::sleep(Duration::from_secs(600));
      }
  });
}
