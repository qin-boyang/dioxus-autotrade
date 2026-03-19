use std::collections::HashMap;
use std::{fs};
use dioxus::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppConfig {
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
}

// Defining the global signal for Dioxus 0.7
pub static CONFIG: GlobalSignal<AppConfig> = Signal::global(|| AppConfig {
    base_url: "待更新".to_string(),
    api_key: "待更新".to_string(),
    api_secret: "待更新".to_string(),
});

pub fn load_global_config() {
    // 獲取當前使用者的 Home 目錄
    let mut config_path = match home::home_dir() {
        Some(path) => path,
        None => panic!("Cannot find home directory"),
    };

    // 拼接成 ~/dioxus-autotrade.config
    config_path.push("dioxus-autotrade.config");

    println!("嘗試讀取絕對路徑: {:?}", config_path);

    // Matching the Result of the file read
    let content = fs::read_to_string(config_path).unwrap_or_else(|_| "reading file error".to_string());

    // Processing into key-value pairs
    let kv_pairs: HashMap<String, String> = content
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();

    *CONFIG.write() = AppConfig {
        base_url: kv_pairs.get("base_url").unwrap_or(&"https://testnet.binance.vision".to_string()).to_string(),
        api_key: kv_pairs.get("api_key").unwrap_or(&"9Up87GEIzsfmHhZXmUmMz7rNynChodTx5mv8Z7X21vMDqpK0RXqyAk2nNmdgqNgZ".to_string()).to_string(),
        api_secret: kv_pairs.get("api_secret").unwrap_or(&"6JZxkyfRBRFBB5V8JpDuJGDLJIacoUQ1adlda0cjfhXJgXQDPBylPsXJFvj5cpqQ".to_string()).to_string(),
    };
}