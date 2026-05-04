use std::time::{SystemTime, UNIX_EPOCH};
use dioxus::prelude::ReadableExt;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;
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
    locked: String,
}
pub(crate) async fn get_balances() -> Result<(f64, f64, f64), Box<dyn std::error::Error>> {
    println!("🦀 Getting USDT balance...");

    let app_config = CONFIG.read();
    println!("🦀 API base_url: {}", app_config.base_url);

    let client = reqwest::Client::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let query = format!("timestamp={}", timestamp);
    let signature = sign_request(&query, &app_config.api_secret);
    println!("🦀 API signature: {}", signature);

    let url = format!("{}/api/v3/account?{}&signature={}", &app_config.base_url, query, signature);
    println!("🦀 url: {}", url);
    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&app_config.api_key).unwrap());
    let res: Response = client.get(url).headers(headers).send().await?;

    // Check if the status is a success (200-299)
    if !&res.status().is_success() {
        let error_text = &res.text().await?;
        println!("❌ API Error Body: {}", error_text);
        return Err(format!("Binance API Error: ").into());
    }
    let body_text = res.text().await?;
    println!("✅ RAW JSON FROM BINANCE: {}", body_text);
    let account: AccountInfo = serde_json::from_str(&body_text)?;

    account.balances.iter().for_each(|b| {
        if b.free.parse::<f64>().unwrap() == 0.0 && b.locked.parse::<f64>().unwrap() == 0.0{
            // println!("❌ asset {:?}", b);
        } else {
            println!("✅ asset {:?}", b);
        }
    });

    let usdt = account.balances.iter()
        .find(|b| b.asset == "USDT")
        .map(|b| b.free.clone())
        .unwrap()
        .parse::<f64>()?;
    let btc = account.balances.iter()
        .find(|b| b.asset == "BTC")
        .map(|b| b.free.clone())
        .unwrap()
        .parse::<f64>()?;
    let eth = account.balances.iter()
        .find(|b| b.asset == "ETH")
        .map(|b| b.free.clone())
        .unwrap()
        .parse::<f64>()?;
    Ok((usdt, btc, eth))
}