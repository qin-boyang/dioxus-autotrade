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

#[derive(Default)]
struct Order {
    target_btc_buy_price: String,
    target_btc_buy_amount: String,
    target_btc_sell_price: String,
    target_btc_sell_amount: String,
}

/// Helper to sign requests for Binance
fn sign_request(query: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}
async fn get_balances() -> Result<(String, String, String), Box<dyn std::error::Error>> {
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
async fn buy_btc_market(quote_order_qty: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut usdt_balance = use_signal(|| "-0.00".to_string());
    let mut btc_balance = use_signal(|| "-0.00".to_string());
    let mut eth_balance = use_signal(|| "-0.00".to_string());
    let btc_price = use_signal(|| "-0.00".to_string());
    let eth_price = use_signal(|| "-0.00".to_string());

    let mut auto_trade_buy_btc = use_signal(|| false);
    let mut auto_trade_sell_btc = use_signal(|| false);
    let mut btc_order = use_signal(|| Order {
        .. Default::default()
    });

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
                p { "USDT 余额: {usdt_balance}" }
                p { "BTC 余额: {btc_balance}" }
                p { "ETH 余额: {eth_balance}" }
                button {
                    class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        if let Ok(new_balance) = get_balances().await {
                            usdt_balance.set(new_balance.0);
                            btc_balance.set(new_balance.1);
                            eth_balance.set(new_balance.2);
                        }
                    },
                    "刷新 USDT BTC ETH 余额"
                }
            }
            section { class: "flex flex-col gap-4 bg-black p-6 rounded-xl border",
                p { "自动成交规则：当价格低于XXX 自动买入BTC" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_order.read().target_btc_buy_price}",
                    oninput: move |evt| btc_order.write().target_btc_buy_price = evt.value()
                }
                p { "自动成交规则：每次买入XXX 自动买入BTC" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_order.read().target_btc_buy_amount}",
                    oninput: move |evt| btc_order.write().target_btc_buy_amount = evt.value()
                }
                p {
                    "機器人執行狀態：{auto_trade_buy_btc.read()}"
                }

                button{
                    class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        auto_trade_buy_btc.set(true);
                        while *auto_trade_buy_btc.read() {
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            println!("开始自动买入BTC");
                            println!("当前价格：{btc_price}");
                            println!("目标价格：{}", btc_order.read().target_btc_buy_price);
                            if btc_price.read().parse::<f64>().unwrap() < btc_order.read().target_btc_buy_price.parse::<f64>().unwrap() {
                                // TODO call function to buy BTC from Binance API
                            }
                        }
                    },
                    "开始自动买入BTC"
                }
                button{
                    class: "bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        auto_trade_buy_btc.set(false);
                        println!("停止自动买入BTC");
                    },
                    "停止自动买入BTC"
                }
            }
        }
    }
}