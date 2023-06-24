use std::{ collections::HashMap };
use types::Response;
use types::client;

#[tauri::command]
pub async fn register(username: String, password: String) -> Result<Response, String> {
    // Make a post request to the server
    let url = "http://localhost:3000/register";
    let mut map = HashMap::new();
    map.insert("username".to_string(), username);
    map.insert("password".to_string(), password);
    let client = client().build().unwrap();
    let res = client.post(url).query(&map).send().await;

    match res {
        Ok(res) => {
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
            }
            return data;
        }
        Err(err) => Err(err.to_string()),
    }
}
