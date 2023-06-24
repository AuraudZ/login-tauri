use std::borrow::BorrowMut;
use std::sync::Arc;
use std::{ collections::HashMap };
use chrono::prelude::*;
use reqwest::Client;
use reqwest::cookie::Jar;
use types::Response;
use std::time::{ UNIX_EPOCH, Duration };

// Global client instance
pub fn client() -> reqwest::Client {
    return reqwest::Client
        ::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(Jar::default()))
        .build()
        .unwrap();
}

#[tauri::command]
pub async fn login(
    window: tauri::Window,
    username: String,
    password: String
) -> Result<Response, String> {
    // Make a post request to the server
    let url = "http://localhost:3000/login";
    let mut map = HashMap::new();
    map.insert("username".to_string(), username);
    map.insert("password".to_string(), password);
    let real_client = client();
    let res = real_client.post(url).query(&map).send().await;

    match res {
        Ok(res) => {
            let qid_cookie = res
                .cookies()
                .find(|c| c.name() == "qid")
                .unwrap();

            let jar = Jar::default();

            let cookie_str = format!("{}={}", qid_cookie.name(), qid_cookie.value());

            jar.add_cookie_str(&cookie_str, &reqwest::Url::parse("http://localhost:3000").unwrap());

            println!("Cookie: {:?}", qid_cookie);
            // Forward the cookie to the client

            let data = res.json::<Response>().await.map_err(|err| err.to_string());

            println!("Data: {:?}", data);
            if data.as_ref().unwrap().error != "" {
                println!("Error: {:?}", data.as_ref().unwrap().error);
                return data;
            }
            if data.is_ok() {
                let data = data.as_ref().unwrap();

                let user = data.user.as_ref().unwrap();

                println!("User: {:?}", user);
                // Check if the user has an active license
                // Convert the timestamp string into an i64
                let timestamp = user.license_expiry.parse::<u64>().unwrap();
                let d = UNIX_EPOCH + Duration::from_secs(timestamp);

                // Create a NaiveDateTime from the timestamp
                let datetime = DateTime::<Utc>::from(d);
                let now = chrono::Utc::now();

                // Check if the license has expired
                if now > datetime {
                    println!("License has expired");
                    window.set_title("License has expired").unwrap();
                    let res = window.emit(
                        "licenseExpired",
                        format!("License has expired at {:?}", datetime)
                    );
                    match res {
                        Ok(_) => println!("Emitted event"),
                        Err(err) => println!("Error: {:?}", err),
                    }
                }
            }
            return data;
        }
        Err(err) => Err(err.to_string()),
    }
}
