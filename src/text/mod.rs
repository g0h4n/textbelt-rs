// FIXME: code impl TextbeltClient and function text()
// maybe text() function will return JSON with textId ?

use super::TextbeltClient;
use serde_json::{Value, json};
use reqwest::Client;

impl <'a> TextbeltClient<'a> {
    pub async fn text(self ,phone: &str, message: &str) -> serde_json::Value {
        let url = format!("{}/text", self.endpoint);
        let client = Client::new()
            .post(&url)
            .json(&json!({
                "phone": phone,
                "message": message,
                "key": self.api_key,
            }))
            .send()
            .await
            .unwrap();
        client.json().await.unwrap()
    }
}
