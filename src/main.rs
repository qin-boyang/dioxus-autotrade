use dioxus::prelude::*;

// 1. Declare the module
mod components {
    pub mod buy;
    pub mod sell;
    pub mod settings;
    pub mod nav_layout;
}
mod config {
    pub mod app_config;
}

mod model {
    pub mod sign_signature;
    pub mod buy_btc;
    pub mod sell_btc;
    pub mod get_balance;
    pub mod get_ticker_price;
    pub mod alarm;
}

// 2. Bring the component into scope
use components::buy::Buy;
use components::sell::Sell;
use components::settings::Settings;
use components::nav_layout::NavLayout;
use config::app_config::load_global_config;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Force set the environment variable programmatically BEFORE anything else
    std::env::set_var("RUST_LOG", "info,h2=warn,hyper=warn,reqwest=warn,rustls=warn");

    // Use the environment variable we just set
    env_logger::init();

    launch(App);
}

#[component]
fn App() -> Element {
    load_global_config();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}

    }
}

#[component]
pub fn Passcode() -> Element {
    // 1. Create a signal to track the input value
    let mut input_text = use_signal(|| String::new());
    const PASSCODE: &str = "123456";
    // 2. Logic to determine what message to show
    let message = match input_text().as_str() {
        "" => "开机密码为预设密码",
        PASSCODE => "密码匹配 ✅",
        _ => "密码不匹配 ❌",
    };
    let nav = use_navigator();

    rsx! {
        div { id: "passcode", class: "p-8 flex flex-col gap-4",
            label { "请输入开机密码" }
            input {
                class: "border p-2 rounded",
                r#type: "text",
                placeholder: "最简单的6位密码",
                // 3. Bind the value and update the signal on change
                value: "{input_text}",
                oninput: move |evt| input_text.set(evt.value())
            }

            // 4. Display the dynamic message
            p {
                class: if input_text() == PASSCODE { "text-green-600" }
                        else if input_text() == "" { "text-grey-600" }
                        else { "text-red-600"},
                "{message}"
            }

            // 5. Display the dynamic button to Start
            if input_text() == PASSCODE {
                button {
                    class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition-colors",
                    onclick: move |_| {
                        nav.push(Route::Buy {});
                    },
                    "开机"
                }
            }
        }
    }
}

// The Routable derive comes from the router feature
#[derive(Routable, Clone, PartialEq, Debug)]
enum Route {
    #[route("/")]
    Passcode {},

    // Correctly nest these inside the layout
    #[layout(NavLayout)]
    #[route("/home")]
    Buy {},
    #[route("/sell")]
    Sell {},
    #[route("/settings")]
    Settings {},
}