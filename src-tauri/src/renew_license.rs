// remember to call `.manage(MyState::default())`
use types::client;
#[tauri::command]
pub async fn renew_license() -> Result<(), String> {
    // Make a post request to the server
    let url = "http://localhost:3000/gen";

    let res = client().build().unwrap().post(url).send().await;

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
