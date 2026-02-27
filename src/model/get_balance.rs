use std::time::{SystemTime, UNIX_EPOCH};
use dioxus::prelude::ReadableExt;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use crate::config::app_config::CONFIG;
use crate::model::sign_signature::sign_request;

#[derive(Deserialize, Debug)]
struct AccountInfo {
    balances: Vec<AssetBalance>,
}

#[derive(Deserialize, Debug, Clone)]
struct AssetBalance {
    asset: String,
    free: String,
}
pub(crate) async fn get_balances() -> Result<(String, String, String), Box<dyn std::error::Error>> {
    println!("🦀 Getting USDT balance...");

    let app_config = CONFIG.read();

    println!("🦀 base_url: {}", app_config.base_url);
    println!("🦀 api_key: {}", app_config.api_key);
    println!("🦀 api_secret: {}", app_config.api_secret);

    let client = reqwest::Client::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let query = format!("timestamp={}", timestamp);
    let signature = sign_request(&query, &app_config.api_secret);

    let url = format!("{}/api/v3/account?{}&signature={}", &app_config.base_url, query, signature);
    println!("🦀 url: {}", url);
    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&app_config.api_key).unwrap());
    println!("🦀 Looking into headers {:?}", headers);
    let res = client.get(url).headers(headers).send().await?;
    println!("🦀 Status: {}", res.status());

    // Check if the status is a success (200-299)
    if !&res.status().is_success() {
        let error_text = &res.text().await?;
        println!("❌ API Error Body: {}", error_text);
        return Err(format!("Binance API Error: ").into());
    }
    let account: AccountInfo = res.json().await?;
    println!("🦀 AccountInfo: {:?}", account);

    let usdt = account.balances.iter()
        .find(|b| b.asset == "USDT")
        .map(|b| b.free.clone())
        .unwrap_or_else(|| "0.0".to_string());
    let btc = account.balances.iter()
        .find(|b| b.asset == "BTC")
        .map(|b| b.free.clone())
        .unwrap_or_else(|| "0.0".to_string());
    let eth = account.balances.iter()
        .find(|b| b.asset == "ETH")
        .map(|b| b.free.clone())
        .unwrap_or_else(|| "0.0".to_string());
    Ok((usdt, btc, eth))
}