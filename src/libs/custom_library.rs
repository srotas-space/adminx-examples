// libs/custom_library.rs
use actix_web::{HttpRequest, Error};
use std::env;
use serde_json::{json, Value};
use log::warn;


/// Extracts the `X-PLATFORM` header from the request or defaults to "iOS/Android"
pub async fn get_x_platform(req: HttpRequest) -> Result<String, Error> {
    let x_platform = req
        .headers()
        .get("X-PLATFORM")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("iOS/Android")
        .to_string();
    Ok(x_platform)
}

/// Extracts the remote IP address from the request or returns "unknown"
pub async fn get_remote_ip_address(req: HttpRequest) -> Result<String, Error> {
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(client_ip) = forwarded_str.split(',').next() {
                println!("**********************************************");
                println!("Found IP x-forwarded-for: {:?}", client_ip);
                println!("**********************************************");
                return Ok(client_ip.trim().to_string());
            }
        }
    }

    let ip_address = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("**********************************************");
    println!("Found IP x-forwarded-for Fallback: {:?}", ip_address);
    println!("**********************************************");
    Ok(ip_address)
}


/// Extracts geo location from IP
pub async fn get_geolocation_from_ip_address(ip_address: String) -> Result<serde_json::Value, Error> {
    let token = env::var("IPINFO_TOKEN").expect("IPINFO_TOKEN must be set");

    let url = format!("https://ipinfo.io/{}?token={}", ip_address, token);

    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let json: Value = response
        .json()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(json)
}


pub async fn get_country_code_from_request(req: HttpRequest) -> Result<serde_json::Value, Error> {
    let ip = get_remote_ip_address(req.clone()).await?;

    let geolocation = match get_geolocation_from_ip_address(ip).await {
        Ok(val) => Some(val),
        Err(e) => {
            warn!("⚠️ Failed to fetch geolocation: {:?}", e);
            None
        }
    };
    println!("geolocation: {:?}", geolocation);
    let country_code = geolocation
        .as_ref()
        .map_or_else(|| "IN".to_string(), |geo| geo["country"].as_str().unwrap_or("IN").to_string());

    Ok(json!({ "country_code": country_code }))
}