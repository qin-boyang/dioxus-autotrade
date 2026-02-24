use dioxus::prelude::*;
use crate::config::app_config::CONFIG;

#[component]
pub fn Settings() -> Element {
    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col gap-8 p-4",
            header {
                h1 { class: "text-2xl font-bold", "设置" }
                p { class: "text-gray-300", "币安的配置 (修改前必须关掉交易机器人)" }
            }

            section { class: "flex flex-col gap-4 bg-black p-6 rounded-xl border",
                // Base URL
                div { class: "flex flex-col gap-2",
                    label { class: "text-sm font-medium", "Base URL" }
                    input {
                        class: "border p-2 rounded",
                        value: "{CONFIG.read().base_url}",
                        oninput: move |evt| CONFIG.write().base_url = evt.value()
                    }
                }

                // API Key
                div { class: "flex flex-col gap-2",
                    label { class: "text-sm font-medium", "API Key" }
                    input {
                        class: "border p-2 rounded",
                        value: "{CONFIG.read().api_key}",
                        oninput: move |evt| CONFIG.write().api_key = evt.value()
                    }
                }

                // API Secret
                div { class: "flex flex-col gap-2",
                    label { class: "text-sm font-medium", "API Secret" }
                    input {
                        class: "border p-2 rounded",
                        value: "{CONFIG.read().api_secret}",
                        oninput: move |evt| CONFIG.write().api_secret = evt.value()
                    }
                }
            }
        }
    }
}