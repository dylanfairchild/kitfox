// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use zbus::{dbus_proxy, Connection};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[dbus_proxy(
    interface = "org.fairchildtech.KitfoxResourceService",
    default_service = "org.fairchildtech.Kitfox",
    default_path = "/org/fairchildtech/Kitfox"
)]
trait ResourceService {
    async fn scan(&mut self) -> zbus::Result<String>;
    async fn request(&self, payload: &str) -> zbus::Result<String>;
}

// Note that async functions need to return a Result to be found.
#[tauri::command]
async fn scan() -> String {
    println!("I was invoked from JS");
    let Ok(connection) = Connection::system().await else {
        return String::from("");
    };
    let Ok(mut proxy) = ResourceServiceProxy::new(&connection).await  else {
        return String::from("");
    };
    let Ok(reply) = proxy.scan().await else {
        return String::from("");
    };

    reply
}

#[tauri::command]
async fn request(request_payload: String) -> String {
    println!("Request invoked from JS- {}", request_payload);
    let Ok(connection) = Connection::system().await else {
        return String::from("");
    };
    let Ok(mut proxy) = ResourceServiceProxy::new(&connection).await else {
        return String::from("");  
    };
    let Ok(reply) = proxy.request(&request_payload).await else {
        return String::from("");
    };
    println!("Reply: {}", reply);
    reply
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan, request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
