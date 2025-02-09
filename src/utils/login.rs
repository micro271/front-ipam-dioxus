use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
struct LoginResponse {
    pub data: TokenData,
    pub status: i32,
    pub success: bool,
}

#[derive(Deserialize, Serialize)]
struct TokenData {
    pub token: String,
}

pub async fn login(user: User) -> Result<String, String> {
    let client = reqwest::Client::new()
        .post("http://localhost:3000/login")
        .body(serde_json::json!(user).to_string())
        .header(reqwest::header::CONTENT_TYPE, "application/json");
    let token = client
        .send()
        .await
        .map(|x| x.json::<LoginResponse>())
        .map_err(|_| "Login error".to_string())?
        .await
        .map_err(|e| e.to_string())?
        .data
        .token;

    let tmp = web_sys::window().expect("Error");
    if let Ok(Some(st)) = tmp.local_storage() {
        st.set("jwt", &token)
            .map_err(|_| "Error to storage the jwt".to_string())?;

        Ok(token)
    } else {
        Err("LocalStorage error".to_string())
    }
}
