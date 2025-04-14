use crate::config;

use serde::{Deserialize, Serialize};
use ureq;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub name: String,
    pub clouds: Clouds,
    pub wind: Wind,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Wind {
    speed: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Clouds {
    all: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Main {
    pub temp: f64,
    pub humidity: u8,
    pub pressure: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Weather {
    pub description: String,
    pub icon: String,
}

#[tauri::command]
pub fn fetch_weather(city: String) -> Result<WeatherResponse, String> {
    let lang = match config::get("lang") {
        Some(v) => v.as_str().unwrap().to_string(),
        None => {
            return Err(config::get_lang());
        }
    };

    let api_key = match config::get("api_key") {
        Some(v) => v.as_str().unwrap().to_string(),
        None => {
            return Err("Missing API key".into());
        }
    };

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&lang={}&appid={}&units=metric",
        city, lang, api_key
    );

    let response = ureq::get(&url).call().map_err(|e| e.to_string())?;
    let body = response.into_string().map_err(|e| e.to_string())?;
    let weather: WeatherResponse = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    Ok(weather)
}
