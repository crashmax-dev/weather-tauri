use crate::config;

mod weather;
mod ipwhois;

use std::{thread, time::Duration};
use once_cell::sync::OnceCell;
use log::info;
use tauri;

pub static HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new(); // define a static variable to store the app handle (singleton)

pub struct AppBuidler {
    app: tauri::App,
}

impl AppBuidler {
  pub async fn build() -> Result<Self, tauri::Error> {
      let builder = tauri::Builder::default()
      .plugin(tauri_plugin_store::Builder::new().build())
      .plugin(tauri_plugin_opener::init())
      .setup(|app| {
          // define a closure to setup the app
          HANDLE.get_or_init(|| app.handle());
          info!("Hello from Praises!");
          config::init_config(app);
          // Check First Run
          if config::is_first_run() {
              info!("First run detected");
              // Set First Run
              config::set("init", true);
          }

          // start_weather_worker(app.app_handle().clone());
          Ok(())
      })
      .invoke_handler(tauri::generate_handler![weather::fetch_weather, ipwhois::fetch_ip_whois])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");

      let app = builder.build(tauri::generate_context!())?;
      Ok(Self { app })
  }

  pub fn run(self) {
      self.app.run(|app, event| {
          match event {
              tauri::RunEvent::Exit => {}
              tauri::RunEvent::ExitRequested { .. } => {}
              tauri::RunEvent::WindowEvent { .. } => {}
              tauri::RunEvent::Ready =>
                  match app.get_window("main") {
                      None => {}
                      Some(win) => {
                          let _ = win.show();
                      }
                  }
              _ => {}
          }
      });
  }
}

// pub fn start_weather_worker(app_handle: tauri::AppHandle) {
//     thread::spawn(move || {
//         let result = weather::fetch_weather(
//             String::from("Irkutsk"),
//             String::from("ru"),
//             String::from("4b7f29a8e15af3ec8d463f83ce5dd419"),
//         );
//         if let Ok(data) = result {
//             let _ = app_handle.emit("weather_update", data);
//         }
//         thread::sleep(Duration::from_secs(600));
//     });
// }
