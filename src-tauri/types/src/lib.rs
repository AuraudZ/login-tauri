use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
// Make fields public
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub license: String,
    pub upgraded: bool,
    pub license_expiry: String,
    pub stripe_id: String,
}

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct Response {
    pub error: String,
    pub message: String,
    // Can be null
    pub user: Option<User>,
}

// Create a global reqwest client
pub fn client() -> reqwest::ClientBuilder {
    return reqwest::Client::builder().cookie_store(true);
}
