use std::time::{SystemTime, UNIX_EPOCH};
use dioxus::desktop::wry::http::{HeaderMap, HeaderValue};
use dioxus::prelude::ReadableExt;
use crate::config::app_config::CONFIG;
use crate::model::sign_signature::sign_request;

pub(crate) async fn sell_btc_market(quantity: f64) -> dioxus::Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Placing Market Sell Order for BTC...");

    let app_config = CONFIG.read();
    let client = reqwest::Client::new();

    // 1. 准备参数 （注意：卖出通常指定 quantity (BTC 的数量) 如果你想按成交额卖出，可以将 quantity 替换为 quoteOrderQty）
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let query = format!(
        "symbol=BTCUSDT&side=SELL&type=MARKET&quantity={}&timestamp={}",
        quantity, timestamp
    );

    // 2. 签名 (逻辑与买入一致)
    let signature = sign_request(&query, &app_config.api_secret);
    let url = format!("{}/api/v3/order?{}&signature={}", &app_config.base_url, query, signature);

    // 3. 设置 Header
    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&app_config.api_key)?);

    // 4. 发送 POST 请求
    let res_result = client.post(&url)
        .headers(headers)
        .send()
        .await;
    // 检查是否是网络/底层请求错误
    let res = match res_result {
        Ok(response) => response,
        Err(e) => {
            // 这里处理的是：DNS 解析不了、连接超时、本地代理拦截等问题
            println!("❌ [Network Error] 无法触达币安服务器: {:?}", e);
            return Err(Box::new(e));
        }
    };

    // 5. 结果处理
    if !res.status().is_success() {
        let status = res.status();
        let error_text = res.text().await.unwrap_or_else(|_| "无法读取错误详情".to_string());
        println!("❌ [币安业务错误] 状态码: {}, 详情: {}", status, error_text);
        return Err(format!("Binance Sell Error ({}): {}", status, error_text).into());
    }

    let response_json: serde_json::Value = res.json().await?;
    println!("✅ Sell Order Success! Details: {:?}", response_json);

    Ok(())
}