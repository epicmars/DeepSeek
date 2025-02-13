use crate::utils;
use tauri::{App, Manager, Url, WebviewUrl, WebviewWindowBuilder};
use reqwest;
use serde_json::{self, Value};
use std::{error::Error, path::PathBuf, str::FromStr};

const DEFAULT_URL: &str = "https://chat.deepseek.com/";

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let conf = utils::get_tauri_conf().unwrap();
    let mut serveUrl = DEFAULT_URL.to_string();
    // println!("use url: {}", serveUrl);
    WebviewWindowBuilder::new(app, "core", WebviewUrl::External(Url::parse(&serveUrl).unwrap()))
        .title("DeepSeek")
        .resizable(true)
        .fullscreen(false)
        .initialization_script(include_str!("../core.js"))
        // .initialization_script(&utils::user_script())
        .inner_size(800.0, 600.0)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .build()?;

    Ok(())
}
