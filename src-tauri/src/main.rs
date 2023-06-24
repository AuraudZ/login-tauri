// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{ collections::HashMap, hash::Hash };

use serde::Serialize;
use serde::Deserialize;
use chrono::prelude::*;
use std::time::{ SystemTime, UNIX_EPOCH, Duration };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    license: String,
    upgraded: bool,
    licenseExpiry: String,
    stripeId: String,
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
struct Response {
    error: String,
    message: String,
    // Can be null
    user: Option<User>,
}

#[tauri::command]
async fn login(
    window: tauri::Window,
    username: String,
    password: String
) -> Result<Response, String> {
    // Make a post request to the server
    let url = "http://localhost:3000/login";
    let mut map = HashMap::new();
    map.insert("username".to_string(), username);
    map.insert("password".to_string(), password);

    let res = reqwest::Client::new().post(url).query(&map).send().await;

    match res {
        Ok(res) => {
            let data = res.json::<Response>().await.map_err(|err| err.to_string());
            println!("Data: {:?}", data);
            if data.as_ref().unwrap().error != "" {
                println!("Error: {:?}", data.as_ref().unwrap().error);
                return data;
            }
            if data.is_ok() {
                let user = data.as_ref().unwrap().user.as_ref().unwrap();
                println!("User: {:?}", user);
                // Check if the user has an active license
                // Convert the timestamp string into an i64
                let timestamp = user.licenseExpiry.parse::<u64>().unwrap();
                let d = UNIX_EPOCH + Duration::from_secs(timestamp);

                // Create a NaiveDateTime from the timestamp
                let datetime = DateTime::<Utc>::from(d);
                let now = chrono::Utc::now();
                // Convert the license expiry date to a DateTime object

                // Check if the license has expired
                if now > datetime {
                    println!("License has expired");

                    window.emit("licenseExpired", format!("License has expired at {:?}", datetime));
                }

                println!("Now: {:?}", now);
            }
            return data;
        }
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
