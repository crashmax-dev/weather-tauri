// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod weather;
mod ipwhois;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
  app::AppBuidler::build().await?.run();
  Ok(())
}
