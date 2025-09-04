// FIXME: code impl TextbeltClient and function status()
use super::{TextbeltClient, Value};
use reqwest::Client;
impl <'a> TextbeltClient<'a> {
    pub async fn status(self, text_id: &str) -> Value{
        let url = format!("{}/status/{text_id}",self.endpoint);
        let client = Client::new()
            .get(&url)
            .send()
            .await
            .unwrap();
        client.json().await.unwrap()
    }
}