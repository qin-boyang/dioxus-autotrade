use dioxus::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use crate::config::app_config::CONFIG;

type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize, Debug)]
struct AccountInfo {
    balances: Vec<AssetBalance>,
}

#[derive(Deserialize, Debug, Clone)]
struct AssetBalance {
    asset: String,
    free: String,
}

// 币安行情数据结构
#[derive(Deserialize, Debug)]
struct PriceTicker {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "c")]
    price: String,
}

/// Helper to sign requests for Binance
fn sign_request(query: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}
async fn get_usdt_balance() -> Result<String, reqwest::Error> {
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

    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&app_config.api_key).unwrap());

    let res = client.get(url).headers(headers).send().await?;
    let account: AccountInfo = res.json().await?;

    let usdt = account.balances.iter()
        .find(|b| b.asset == "USDT")
        .map(|b| b.free.clone())
        .unwrap_or_else(|| "0.0".to_string());

    Ok(usdt)
}
fn get_ticker_price(mut btc_price: Signal<String>, mut eth_price: Signal<String>) {
    let _price_task = use_coroutine(move |_rx: UnboundedReceiver<()>| {
        async move {
            let url = "wss://stream.binance.com:9443/ws/btcusdt@ticker/ethusdt@ticker";

            if let Ok((mut ws_stream, _)) = connect_async(url).await {
                while let Some(Ok(msg)) = ws_stream.next().await {
                    if let Message::Text(text) = msg {
                        if let Ok(ticker) = serde_json::from_str::<PriceTicker>(&text) {
                            let formatted = ticker.price
                                .parse::<f64>()
                                .map(|p| format!("{:.2}", p))
                                .unwrap_or(ticker.price);

                            match ticker.symbol.as_str() {
                                "BTCUSDT" => btc_price.set(formatted),
                                "ETHUSDT" => eth_price.set(formatted),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    });
}

// UI Component
#[allow(non_snake_case)]
pub fn Home() -> Element {
    let mut balance = use_signal(|| "-0.00".to_string());
    let btc_price = use_signal(|| "-0.00".to_string());
    let eth_price = use_signal(|| "-0.00".to_string());

    get_ticker_price(btc_price, eth_price);

    rsx! {

        div { class: "flex flex-col gap-8 p-4",
            header {
                h1 { class: "text-2xl font-bold", "首页" }
                p { class: "text-gray-300", "币安交易机器人" }
            }
            // 实时行情卡片
            section { class: "grid grid-cols-2 gap-4",
                div { class: "bg-gray-800 p-4 rounded-xl border border-gray-700",
                    p { class: "text-sm text-gray-400", "BTC/USDT" }
                    p { class: "text-xl font-mono text-orange-400", "§ {btc_price}" }
                }
                div { class: "bg-gray-800 p-4 rounded-xl border border-gray-700",
                    p { class: "text-sm text-gray-400", "ETH/USDT" }
                    p { class: "text-xl font-mono text-purple-400", "§ {eth_price}" }
                }
            }
            section { class: "flex flex-col gap-4 bg-black p-6 rounded-xl border",
                p { "USDT 余额: {balance}" }
                button {
                    class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        if let Ok(new_balance) = get_usdt_balance().await {
                            balance.set(new_balance);
                        }
                    },
                    "刷新 USDT 余额"
                }
            }
        }
    }
}