use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Settings() -> Element {
    let mut auto_trade = use_signal(|| false);
    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col gap-8",
            header {
                h1 { class: "text-2xl font-bold text-gray-800", "Settings" }
                p { class: "text-gray-500", "Configure your trading bot" }
            }

            section { class: "flex flex-col gap-4 bg-white p-6 rounded-xl shadow-sm border border-gray-100",
                h2 { class: "text-lg font-semibold border-b pb-2", "General" }

                // Toggle Switch
                div { class: "flex items-center justify-between py-2",
                    span { "Enable Auto-Trading" }
                    button {
                        class: if auto_trade() { "bg-green-500 w-12 h-6 rounded-full relative transition-colors" } else { "bg-gray-300 w-12 h-6 rounded-full relative transition-colors" },
                        onclick: move |_| auto_trade.set(!auto_trade()),
                        div {
                            class: if auto_trade() { "absolute right-1 top-1 bg-white w-4 h-4 rounded-full transition-all" } else { "absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-all" }
                        }
                    }
                }

                div { class: "flex flex-col gap-2 py-2",
                    label { class: "text-sm text-gray-600", "API Key" }
                    input {
                        r#type: "password",
                        class: "border p-2 rounded bg-gray-50",
                        value: "**************************"
                    }
                }
            }

            // Logout Button
            button {
                class: "mt-4 text-red-500 font-semibold hover:bg-red-50 p-2 rounded transition",
                onclick: move |_| { nav.push(Route::Passcode {}); },
                "Logout and Lock App"
            }
        }
    }
}