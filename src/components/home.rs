use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 animate-in fade-in duration-500",
            // Header Section
            header { class: "flex justify-between items-end",
                div {
                    h1 { class: "text-3xl font-bold text-slate-900", "Dashboard" }
                    p { class: "text-slate-500", "Welcome back, trader." }
                }
                span { class: "text-sm font-mono bg-slate-100 p-2 rounded", "UTC: 12:45:02" }
            }

            // Summary Cards Row
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                SummaryCard {
                    title: "Portfolio Value",
                    value: "$124,500.00",
                    trend: "+$1,200",
                    is_positive: true
                }
                SummaryCard {
                    title: "Active Bots",
                    value: "4",
                    trend: "Running",
                    is_positive: true
                }
                SummaryCard {
                    title: "Daily Profit",
                    value: "$432.50",
                    trend: "-12%",
                    is_positive: false
                }
            }

            // Chart Placeholder
            div { class: "bg-white p-6 rounded-2xl shadow-sm border border-slate-200 h-64 flex items-center justify-center",
                div { class: "text-center",
                    p { class: "text-slate-400 italic", "Chart engine loading..." }
                    div { class: "mt-4 flex gap-2 justify-center",
                        for _ in 0..5 {
                            div { class: "w-2 h-8 bg-blue-100 rounded-full animate-pulse" }
                        }
                    }
                }
            }

            // Recent Activity
            div { class: "bg-white p-6 rounded-2xl shadow-sm border border-slate-200",
                h2 { class: "text-lg font-semibold mb-4", "Recent Alerts" }
                ul { class: "divide-y divide-slate-100",
                    ActivityItem { msg: "BTC/USD Buy Order Filled", time: "2 mins ago" }
                    ActivityItem { msg: "Volatility Spike Detected in ETH", time: "15 mins ago" }
                    ActivityItem { msg: "Daily Report Generated", time: "1 hour ago" }
                }
            }
        }
    }
}

#[component]
fn SummaryCard(title: String, value: String, trend: String, is_positive: bool) -> Element {
    let trend_color = if is_positive { "text-green-500" } else { "text-red-500" };

    rsx! {
        div { class: "bg-white p-5 rounded-2xl shadow-sm border border-slate-200 flex flex-col gap-1",
            span { class: "text-sm text-slate-500 font-medium", "{title}" }
            span { class: "text-2xl font-bold text-slate-900", "{value}" }
            span { class: "text-xs font-semibold {trend_color}", "{trend}" }
        }
    }
}

#[component]
fn ActivityItem(msg: String, time: String) -> Element {
    rsx! {
        li { class: "py-3 flex justify-between items-center",
            span { class: "text-sm text-slate-700", "{msg}" }
            span { class: "text-xs text-slate-400", "{time}" }
        }
    }
}