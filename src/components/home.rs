use dioxus::prelude::*;

use crate::model::buy_btc::buy_btc_market;
use crate::model::sell_btc::sell_btc_market;
use crate::model::get_balance::get_balances;
use crate::model::get_ticker_price::get_ticker_price;

// 币安行情数据结构


#[derive(Default)]
struct Order {
    target_btc_buy_price: String,
    quote_order_qty: String, // 即你想花多少 USDT
    target_btc_sell_price: String,
    sell_quantity: String,
}

// UI Component
#[allow(non_snake_case)]
pub fn Home() -> Element {
    // 实时行情
    let btc_price = use_signal(|| "-0.00".to_string());
    let eth_price = use_signal(|| "-0.00".to_string());

    // 获取余额
    let mut usdt_balance = use_signal(|| "-0.00".to_string());
    let mut btc_balance = use_signal(|| "-0.00".to_string());
    let mut eth_balance = use_signal(|| "-0.00".to_string());

    // 自动交易
    let mut auto_trade_buy_btc = use_signal(|| false);
    let mut auto_trade_sell_btc = use_signal(|| false);

    // 订单信息
    let mut btc_order = use_signal(|| Order {
        .. Default::default()
    });

    // 获取行情
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
            // 账户信息
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
            // 自动买入
            section { class: "flex flex-col gap-4 bg-black p-6 rounded-xl border",
                p { "自动成交规则：当价格低于XXX（美元） 自动买入BTC" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_order.read().target_btc_buy_price}",
                    oninput: move |evt| btc_order.write().target_btc_buy_price = evt.value()
                }
                p { "自动成交规则：即你想花多少 USDT 去买入" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_order.read().quote_order_qty}",
                    oninput: move |evt| btc_order.write().quote_order_qty = evt.value()
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
                            println!("ℹ️ 开始自动买入BTC");

                            let current_price_str = btc_price.read().clone();
                            let (target_buy_price_str, quote_order_qty_str) = {
                                let order = btc_order.read();
                                (order.target_btc_buy_price.clone(), order.quote_order_qty.clone())
                            };

                            println!("ℹ️ 当前价格：{}", current_price_str);
                            println!("ℹ️ 目标价格：{}", target_buy_price_str);

                            match (
                                current_price_str.parse::<f64>(),
                                target_buy_price_str.parse::<f64>()
                            ) {
                                (Ok(current_price), Ok(target_price)) => {
                                    if current_price < target_price {
                                        match buy_btc_market(&quote_order_qty_str).await {
                                            Ok(_) => {
                                                println!("✅ 自动买入BTC成功");
                                                auto_trade_buy_btc.set(false);
                                            },
                                            Err(e) => {
                                                println!("❌ 自动买入BTC失败：{}", e);
                                                // Consider breaking the loop on failure?
                                            }
                                        }
                                    }
                                },
                                (Err(e), _) => println!("❌ Failed to parse current price: {}", e),
                                (_, Err(e)) => println!("❌ Failed to parse target price: {}", e),
                            }
                        }
                    },
                    "开始自动买入BTC"
                }
                button{
                    class: "bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        auto_trade_buy_btc.set(false);
                        println!("ℹ️ 停止自动买入BTC");
                    },
                    "停止自动买入BTC"
                }
            }
            // 自动卖出
            section { class: "flex flex-col gap-4 bg-black p-6 rounded-xl border",
                p { "自动成交规则：当价格高于XXX（美元） 自动卖出BTC" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_order.read().target_btc_sell_price}",
                    oninput: move |evt| btc_order.write().target_btc_sell_price = evt.value()
                }
                p { "自动成交规则：即你想卖出多少个 BTC" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_order.read().sell_quantity}",
                    oninput: move |evt| btc_order.write().sell_quantity = evt.value()
                }
                p {
                    "機器人執行狀態：{auto_trade_sell_btc.read()}"
                }

                button{
                    class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        auto_trade_sell_btc.set(true);
                        while *auto_trade_sell_btc.read() {
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            println!("ℹ️ 开始自动卖出BTC");

                            let current_price_str = btc_price.read().clone();
                            let (target_sell_price_str, sell_quantity_str) = {
                                let order = btc_order.read();
                                (order.target_btc_sell_price.clone(), order.sell_quantity.clone())
                            };

                            println!("ℹ️ 当前价格：{}", current_price_str);
                            println!("ℹ️ 目标价格：{}", target_sell_price_str);

                            match (
                                current_price_str.parse::<f64>(),
                                target_sell_price_str.parse::<f64>()
                            ) {
                                (Ok(current_price), Ok(target_price)) => {
                                    if current_price > target_price {
                                        match sell_btc_market(&sell_quantity_str).await {
                                            Ok(_) => {
                                                println!("✅ 自动卖出BTC成功");
                                                auto_trade_sell_btc.set(false);
                                            },
                                            Err(e) => {
                                                println!("❌ 自动卖出BTC失败：{}", e);
                                                // Consider breaking the loop on failure?
                                            }
                                        }
                                    }
                                },
                                (Err(e), _) => println!("❌ Failed to parse current price: {}", e),
                                (_, Err(e)) => println!("❌ Failed to parse target price: {}", e),
                            }
                        }
                    },
                    "开始自动卖出BTC"
                }
                button{
                    class: "bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        auto_trade_sell_btc.set(false);
                        println!("ℹ️ 停止自动卖出BTC");
                    },
                    "停止自动卖出BTC"
                }
            }
        }
    }
}