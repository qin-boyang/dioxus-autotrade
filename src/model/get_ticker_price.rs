use dioxus::prelude::*;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use std::time::Duration;
use crate::model::alarm::play_beep;
// 引入时间包

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

            // 使用 loop 实现自动重试
            loop {
                println!("[WS] 尝试连接 Binance: {}", url);

                match connect_async(url).await {
                    Ok((mut ws_stream, _)) => {
                        println!("[WS] 连接成功！开始接收行情...");

                        while let Some(msg_result) = ws_stream.next().await {
                            match msg_result {
                                Ok(msg) => {
                                    if let Message::Text(text) = msg {
                                        if let Ok(ticker) = serde_json::from_str::<PriceTicker>(&text) {
                                            if let Ok(formatted) = ticker.price.parse::<f64>() {
                                                match ticker.symbol.as_str() {
                                                    "BTCUSDT" => btc_price.set(formatted),
                                                    "ETHUSDT" => eth_price.set(formatted),
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("[Error] 读取消息错误: {}，准备重连...", e);
                                    break; // 跳出内部 while 循环，进入外部 loop 进行重连
                                }
                            }
                        }
                        println!("[WS] 连接已断开。");
                    }
                    Err(e) => {
                        println!("[Error] 无法建立连接: {}。5秒后重试...", e);
                    }
                }

                // 在下一次循环（重试）之前等待一段时间
                println!("[WS] 报警，并尝试重新连接。");
                play_beep();
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    });
}