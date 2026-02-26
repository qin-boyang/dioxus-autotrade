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
    api_key: "9Up87GEIzsfmHhZXmUmMz7rNynChodTx5mv8Z7X21vMDqpK0RXqyAk2nNmdgqNgZ".to_string(),
    api_secret: "6JZxkyfRBRFBB5V8JpDuJGDLJIacoUQ1adlda0cjfhXJgXQDPBylPsXJFvj5cpqQ".to_string(),
});