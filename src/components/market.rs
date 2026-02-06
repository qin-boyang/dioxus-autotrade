use dioxus::prelude::*;

#[component]
pub fn Market() -> Element {
    let assets = use_signal(|| vec![
        ("BTC/USD", "$96,500", "+2.4%"),
        ("ETH/USD", "$2,450", "-1.2%"),
        ("SOL/USD", "$145", "+5.7%"),
    ]);

    rsx! {
        div { class: "flex flex-col gap-6",
            header {
                h1 { class: "text-2xl font-bold text-gray-800", "Market Overview" }
                p { class: "text-gray-500", "Real-time market data" }
            }

            div { class: "bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden",
                table { class: "w-full text-left",
                    thead { class: "bg-gray-50 border-b border-gray-100",
                        tr {
                            th { class: "p-4 text-sm font-semibold text-gray-600", "Asset" }
                            th { class: "p-4 text-sm font-semibold text-gray-600", "Price" }
                            th { class: "p-4 text-sm font-semibold text-gray-600", "24h Change" }
                        }
                    }
                    tbody {
                        for (name, price, change) in assets() {
                            tr { class: "border-b border-gray-50 hover:bg-gray-50 transition",
                                td { class: "p-4 font-medium", "{name}" }
                                td { class: "p-4", "{price}" }
                                td {
                                    class: if change.contains('+') { "p-4 text-green-500" } else { "p-4 text-red-500" },
                                    "{change}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}