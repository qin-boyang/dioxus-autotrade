use chrono::{DateTime, Duration, Local};
use dioxus::prelude::*;
use crate::model::alarm::play_beep;
use crate::model::buy_btc::buy_btc_market;
use crate::model::sell_btc::sell_btc_market;
use crate::model::get_balance::get_balances;
use crate::model::get_ticker_price::get_ticker_price;

// UI Component
#[allow(non_snake_case)]
pub fn Sell() -> Element {
    // 实时行情
    let btc_price: Signal<f64> = use_signal(|| -0.00);
    let eth_price: Signal<f64> = use_signal(|| -0.00);

    // 获取余额
    let mut usdt_balance: Signal<f64> = use_signal(|| -0.00);
    let mut btc_balance: Signal<f64> = use_signal(|| -0.00);
    let mut eth_balance: Signal<f64> = use_signal(|| -0.00);

    // 自动交易
    let mut auto_trade_buy_btc = use_signal(|| false);
    let mut auto_trade_sell_btc = use_signal(|| false);

    // 开始交易时的指标
    let mut btc_starting_price: Signal<String> = use_signal(|| "-0.00".to_string());
    let mut btc_gap_price: Signal<String> = use_signal(|| "-0.00".to_string());
    let mut btc_buy_trigger_price: Signal<String> = use_signal(|| "-0.00".to_string());
    let mut btc_sell_trigger_price: Signal<String> = use_signal(|| "-0.00".to_string());
    let mut btc_sell_qty: Signal<String> = use_signal(|| "-0.00".to_string());
    let mut btc_duration: Signal<String> = use_signal(|| "-0".to_string());
    let mut btc_ending_time: Signal<DateTime<Local>> = use_signal(|| Local::now());
    let mut btc_trade_times: Signal<String> = use_signal(|| "0".to_string());

    // 订单信息
    let mut quote_order_qty: Signal<String> = use_signal(|| "0.00".to_string());
    let mut sell_quantity: Signal<String> = use_signal(|| "0.00".to_string());


    // 获取行情
    get_ticker_price(btc_price, eth_price);

    rsx! {
        div { class: "flex flex-col gap-8 p-4",
            header {
                h1 { class: "text-2xl font-bold", "卖出BTC界面" }
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
            // 自动卖出
            section { class: "flex flex-col gap-4 bg-black p-6 rounded-xl border",
                p { "开始自动交易时的BTC价格" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_starting_price}",
                    readonly: true,
                }
                p { "ℹ️ 跌多少价格卖出BTC（小数点最多5位，须>=0.00008）" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_gap_price}",
                    oninput: move |evt| {
                        btc_gap_price.set(evt.value());
                    }
                }
                p { "触发自动卖出时的BTC价格" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_sell_trigger_price}",
                    readonly: true,
                }
                p { "止损触发自动买入时的BTC价格" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_buy_trigger_price}",
                    readonly: true,
                }
                p { "ℹ️ 追单BTC数量（小数点最多5位，须>=0.00008）" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_sell_qty}",
                    oninput: move |evt| {
                        btc_sell_qty.set(evt.value());
                        sell_quantity.set(btc_sell_qty.read().parse::<f64>().unwrap_or_default().to_string());
                    }
                }
                p { "自动成交规则：即你想卖出多少 BTC" }
                input {
                    class: "border p-2 rounded",
                    value: "{sell_quantity}",
                    readonly: true,
                }
                p { "ℹ️ 操作几分钟（须>=1，不可有小数）" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_duration}",
                    oninput: move |evt| {
                        btc_duration.set(evt.value());
                        btc_ending_time.set(Local::now() + Duration::minutes(btc_duration.read().parse::<i64>().unwrap_or_default()));
                    }
                }
                p { "几点几分结束 (+08:00 为北京时间)" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_ending_time}",
                    readonly: true,
                }
                p { "ℹ️ 平仓强制结束时花多少USDT买入（须>=5 USDT，小数点最多7位）" }
                input {
                    class: "border p-2 rounded",
                    value: "{quote_order_qty}",
                    oninput: move |evt| quote_order_qty.set(evt.value())
                }
                p { "ℹ️ 操作几次" }
                input {
                    class: "border p-2 rounded",
                    value: "{btc_trade_times}",
                    oninput: move |evt| btc_trade_times.set(evt.value())
                }
                p {
                    "机器人状态："
                    br {}
                    if auto_trade_sell_btc.read().clone() {
                        "✅ 自动卖出已经开始"
                    } else {
                        "❌ 自动卖出已经停止"
                    }
                    br {}
                    if auto_trade_buy_btc.read().clone() {
                        "✅ 自动买入已经开始"
                    } else {
                        "❌ 自动买入已经停止"
                    }
                }

                button{
                    class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        println!("ℹ️ 开始自动卖出BTC");
                        'outer: while btc_trade_times.read().parse::<i32>().unwrap_or_default() > 0 {

                            auto_trade_sell_btc.set(true);
                            auto_trade_buy_btc.set(false);
                            btc_starting_price.set(btc_price.read().to_string());
                            btc_sell_trigger_price.set((btc_starting_price.read().parse::<f64>().unwrap_or_default() - btc_gap_price.read().parse::<f64>().unwrap_or_default()).to_string());
                            btc_buy_trigger_price.set((btc_starting_price.read().parse::<f64>().unwrap_or_default() + btc_gap_price.read().parse::<f64>().unwrap_or_default()).to_string());
                            btc_ending_time.set(Local::now() + Duration::minutes(btc_duration.read().parse::<i64>().unwrap_or_default()));
                            println!("ℹ️ 结束时间： {}", btc_ending_time.read());
                            'inner: while auto_trade_sell_btc.read().clone() || auto_trade_buy_btc.read().clone() {
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                if Local::now() >= *btc_ending_time.read() {
                                    if auto_trade_buy_btc.read().clone() {
                                        println!("✅ 时间到，开始强制买入BTC");
                                        match buy_btc_market(quote_order_qty.read().parse::<f64>().unwrap_or_default()).await {
                                            Ok(_) => {
                                                println!("✅ 强制买入BTC成功");
                                                auto_trade_buy_btc.set(false);
                                            },
                                            Err(e) => {
                                                println!("❌❌❌ 强制买入BTC失败：{}", e);
                                                play_beep();
                                                break 'outer;
                                            }
                                        }
                                    }
                                    println!("❌ 自动交易已经结束");
                                    auto_trade_sell_btc.set(false);
                                    auto_trade_buy_btc.set(false);
                                    break 'inner;
                                }

                                if auto_trade_sell_btc.read().clone() {
                                    println!("ℹ️ 当前价格：{}", btc_price.read().clone());
                                    println!("ℹ️ 卖出触发价格：{}", btc_sell_trigger_price.read().clone());
                                    if btc_price.read().clone() <= btc_sell_trigger_price.read().parse::<f64>().unwrap_or_default() {
                                        println!("✅ 触发价格已到，开始自动卖出BTC");
                                        match sell_btc_market(sell_quantity.read().parse::<f64>().unwrap_or_default()).await {
                                            Ok(_) => {
                                                println!("✅ 自动卖出BTC成功");
                                                auto_trade_sell_btc.set(false);
                                                auto_trade_buy_btc.set(true);
                                            },
                                            Err(e) => {
                                                println!("❌❌❌ 自动卖出BTC失败：{}", e);
                                                play_beep();
                                                break 'outer;
                                            }
                                        }
                                    }
                                }

                                if auto_trade_buy_btc.read().clone() {
                                    println!("ℹ️ 当前价格：{}", btc_price.read().clone());
                                    println!("ℹ️ 买入触发价格：{}", btc_buy_trigger_price.read().clone());
                                    if btc_price.read().clone() >= btc_buy_trigger_price.read().parse::<f64>().unwrap_or_default() {
                                        println!("✅ 触发价格已到，开始强制买入BTC");
                                        match buy_btc_market(quote_order_qty.read().parse::<f64>().unwrap_or_default()).await {
                                            Ok(_) => {
                                                println!("✅ 自动买入BTC成功");
                                                auto_trade_buy_btc.set(false);
                                            },
                                            Err(e) => {
                                                println!("❌❌❌ 自动买入BTC失败：{}", e);
                                                play_beep();
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                            }

                            let counter = (btc_trade_times.read().parse::<i32>().unwrap_or_default() - 1).to_string();
                            btc_trade_times.set(counter);
                        }
                    },
                    "开始自动卖出BTC"
                }
                button{
                    class: "bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| async move {
                        auto_trade_buy_btc.set(false);
                        auto_trade_sell_btc.set(false);
                        btc_trade_times.set("0".to_string());
                        println!("ℹ️ 停止自动卖出BTC");
                    },
                    "停止自动卖出BTC"
                }
            }
        }
    }
}