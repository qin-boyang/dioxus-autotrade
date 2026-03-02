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

            if let Ok((mut ws_stream, _)) = connect_async(url).await {
                while let Some(Ok(msg)) = ws_stream.next().await {
                    if let Message::Text(text) = msg {
                        if let Ok(ticker) = serde_json::from_str::<PriceTicker>(&text) {
                            let formatted = ticker.price.parse::<f64>()
                                .unwrap_or(ticker.price.parse::<f64>().unwrap());

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