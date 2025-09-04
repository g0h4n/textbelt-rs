use std::fmt::format;

// FIXME: code impl TextbeltClient and function quota()
use super::{TextbeltClient,Value};
use reqwest::Client;

impl<'a> TextbeltClient<'a> {
    pub async fn quota(self) -> Value{
        let url = format!("{}/quota/{}",self.endpoint,self.api_key);
        let client = Client::new()
            .get(&url)
            .send()
            .await
            .unwrap();
        client.json().await.unwrap()
    }
}