use dioxus::prelude::*;
use crate::Route; // Ensure this points to your main Route enum

#[component]
pub fn NavLayout() -> Element {
    rsx! {
        div { class: "flex flex-col h-screen",
            div { class: "flex-1 overflow-auto",
                // THIS IS THE KEY: Home/Settings/Market render here
                Outlet::<Route> {}
            }

            // Your bottom nav code here...
            nav { class: "h-20 border-t flex justify-around items-center",
                 Link { to: Route::Home {}, "Home" }
                 Link { to: Route::Market {}, "Market" }
                 Link { to: Route::Settings {}, "Settings" }
            }
        }
    }
}