use dioxus::prelude::*;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[derive(Deserialize, Debug)]
struct PriceTicker {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "c")]
    price: String,
}
pub(crate) fn get_ticker_price(mut btc_price: Signal<f64>, mut eth_price: Signal<f64>) {
    let _price_task = use_coroutine(move |_rx: UnboundedReceiver<()>| {
        async move {
            let url = "wss://stream.binance.com:9443/ws/btcusdt@ticker/ethusdt@ticker";

            println!("[WS] 尝试连接 Binance: {}", url);

            match connect_async(url).await {
                Ok((mut ws_stream, _)) => {
                    println!("[WS] 连接成功！开始接收行情...");

                    while let Some(msg_result) = ws_stream.next().await {
                        match msg_result {
                            Ok(msg) => {
                                if let Message::Text(text) = msg {
                                    // 如果你想看最原始的 JSON 字符串，取消下面这行的注释：
                                    // println!("[Raw] {}", text);

                                    match serde_json::from_str::<PriceTicker>(&text) {
                                        Ok(ticker) => {
                                            if let Ok(formatted) = ticker.price.parse::<f64>() {
                                                // 打印解析后的价格，确认逻辑正确
                                                // println!("[Parsed] {} -> {}", ticker.symbol, formatted);

                                                match ticker.symbol.as_str() {
                                                    "BTCUSDT" => btc_price.set(formatted),
                                                    "ETHUSDT" => eth_price.set(formatted),
                                                    _ => {}
                                                }
                                            }
                                        }
                                        Err(e) => println!("[Error] JSON 解析失败: {}. 原始数据: {}", e, text),
                                    }
                                }
                            }
                            Err(e) => println!("[Error] 读取消息错误: {}", e),
                        }
                    }
                    println!("[WS] 连接已断开。");
                }
                Err(e) => println!("[Error] 无法建立连接: {}", e),
            }
        }
    });
}