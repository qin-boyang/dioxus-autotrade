use std::time::{SystemTime, UNIX_EPOCH};
use dioxus::desktop::wry::http::{HeaderMap, HeaderValue};
use dioxus::prelude::ReadableExt;
use crate::config::app_config::CONFIG;
use crate::model::sign_signature::sign_request;

pub(crate) async fn buy_btc_market(quote_order_qty: f64) -> dioxus::Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Placing Market Buy Order for BTC...");

    let app_config = CONFIG.read();
    let client = reqwest::Client::new();

    // 1. 準備參數 (市價單通常建議使用 quoteOrderQty，即你想花多少 USDT)
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let query = format!(
        "symbol=BTCUSDT&side=BUY&type=MARKET&quoteOrderQty={}&timestamp={}",
        quote_order_qty, timestamp
    );

    // 2. 簽名
    let signature = sign_request(&query, &app_config.api_secret);
    let url = format!("{}/api/v3/order?{}&signature={}", &app_config.base_url, query, signature);

    // 3. 設置 Header
    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&app_config.api_key)?);

    // 4. 發送 POST 請求 (注意下單是 POST)
    let res = client.post(url)
        .headers(headers)
        .send()
        .await?;

    // 5. 處理結果
    if !res.status().is_success() {
        let error_text = res.text().await?;
        println!("❌ Order Failed: {}", error_text);
        return Err(format!("Binance Buy Error: {}", error_text).into());
    }

    let response_json: serde_json::Value = res.json().await?;
    println!("✅ Order Success! Details: {:?}", response_json);

    Ok(())
}