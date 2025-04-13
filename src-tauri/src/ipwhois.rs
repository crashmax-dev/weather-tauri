use serde::{Deserialize, Serialize};
use ureq;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IpWhoisResponse {
  pub success: bool,
  pub city: String,
}

#[tauri::command]
pub fn fetch_ip_whois(lang: String) -> Result<IpWhoisResponse, String> {
    let url = format!(
        "https://ipwhois.app/json/?objects=success,city&lang={}",
        lang
    );

    let response = ureq::get(&url).call().map_err(|e| e.to_string())?;
    let body = response.into_string().map_err(|e| e.to_string())?;
    let whois: IpWhoisResponse = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    Ok(whois)
}
