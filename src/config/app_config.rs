use dioxus::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppConfig {
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
}

// Defining the global signal for Dioxus 0.7
pub static CONFIG: GlobalSignal<AppConfig> = Signal::global(|| AppConfig {
    base_url: "https://testnet.binance.vision".to_string(),
    api_key: "EBsp5hk2YFFspJnKyHwcf1SImTDsJZbn1dHY3CwAkAcXjaCOILXEJ2ZbQ6TCwh9c".to_string(),
    api_secret: "some secret".to_string(),
});