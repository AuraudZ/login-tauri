use std::sync::Arc;
use std::{ collections::HashMap };
use chrono::prelude::*;
use reqwest::cookie::Jar;
use reqwest::cookie::CookieStore;
use types::{ Response };
use std::time::{ UNIX_EPOCH, Duration };

pub fn jar() -> Arc<Jar> {
    // Cookie store for the client
    let cookie_store = Arc::new(Jar::default());
    return cookie_store;
}

pub fn client_builder() -> reqwest::ClientBuilder {
    // Examine initial contents
    println!("initial load");

    return reqwest::Client::builder().cookie_provider(std::sync::Arc::clone(&jar()));
}

// Global client instance
pub fn client() -> reqwest::Client {
    // Cookie store for the client
    client_builder().build().unwrap()
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
    let real_client = client_builder();

    let tmp_client = client_builder().build().unwrap();

    let res = tmp_client.post(url).query(&map).send().await;

    match res {
        Ok(res) => {
            let qid_cookie = res
                .cookies()
                .find(|c| c.name() == "qid")
                .unwrap();

            println!("QID Cookie: {:?}", qid_cookie);

            let qid_cookie_str = format!("{}={}", qid_cookie.name(), qid_cookie.value());
            println!("QID Cookie Str: {:?}", qid_cookie_str);

            // Add the cookie to the cookie store
            jar().add_cookie_str(&qid_cookie_str, &url::Url::parse(url).unwrap());
            // Write

            real_client.cookie_provider(std::sync::Arc::clone(&jar())).build().unwrap();
            // println!("Insert: {:?}", insert);
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

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn renew_license() -> Result<(), String> {
    // Make a post request to the server
    let url = "http://localhost:3000/gen";

    let jar_cookies = jar().cookies(&url::Url::parse("http://localhost:3000/login").unwrap());
    println!("Jar Cookies: {:?}", jar_cookies);
    let res = client().post(url).send().await;

    match res {
        Ok(res) => {
            res.headers()
                .iter()
                .for_each(|(key, value)| {
                    println!("{}: {:?}", key, value);
                });
            let data = res.json::<types::Response>().await.map_err(|err| err.to_string());
            println!("Data: {:?}", data);
            if data.as_ref().unwrap().error != "" {
                println!("Error: {:?}", data.as_ref().unwrap().error);
                return Ok(());
            }
            if data.is_ok() {
                let data = data.as_ref().unwrap();
                let user = data.user.as_ref().unwrap();

                println!("User: {:?}", user);
            }
            return Ok(());
        }
        Err(err) => Err(err.to_string()),
    }
}
