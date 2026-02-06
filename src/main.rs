use dioxus::prelude::*;

// 1. Declare the module
mod components {
    pub mod home;
    pub mod settings;
    pub mod nav_layout;
    pub mod market;
}

// 2. Bring the component into scope
use components::home::Home;
use components::settings::Settings;
use components::nav_layout::NavLayout;
use components::market::Market;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
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
        "" => "Please enter the passcode that developer shared",
        PASSCODE => "Success! Access Granted ✅",
        _ => "Incorrect passcode ❌",
    };
    let nav = use_navigator();

    rsx! {
        div { id: "passcode", class: "p-8 flex flex-col gap-4",
            label { "Enter Passcode to Unlock:" }
            input {
                class: "border p-2 rounded",
                r#type: "text",
                placeholder: PASSCODE,
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
                        nav.push(Route::Home {});
                    },
                    "Start Application"
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
    Home {},
    #[route("/settings")]
    Settings {},
    #[route("/market")]
    Market {},
}