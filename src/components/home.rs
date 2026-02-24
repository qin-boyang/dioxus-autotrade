use dioxus::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
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

// UI Component
pub fn Home() -> Element {
    let mut balance = use_signal(|| "".to_string());

    rsx! {

        div { class: "flex flex-col gap-8 p-4",
            header {
                h1 { class: "text-2xl font-bold", "首页" }
                p { class: "text-gray-300", "币安交易机器人" }
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