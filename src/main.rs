#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use web_view::*;

fn main() {
    let html_content = include_str!("index.html");
	
    web_view::builder()
        .title("2021年“一二·九”师生歌会统分")
        .content(Content::Html(html_content))
        // .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}