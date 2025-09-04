use super::TextbeltClient;
use serde_json::Value;
use anyhow::Result;
use reqwest::Client;

impl <'a> TextbeltClient<'a> {
    /// # How to send an text SMS?
    /// <https://docs.textbelt.com/#send-an-sms-using-http-post>
    ///
    /// # Example
    ///
    /// ```rust
    /// use textbelt::TextbeltClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let tc = TextbeltClient::new("Your textbelt API Key", None);
    ///     let phone = "+33601020304";
    ///     let message = "Hello from textbelt-rs!\nNoodle/g0h4n";
    ///     match tc.text(&phone, &message).await {
    ///         Ok(res) => {
    ///             println!("{:?}", &res);
    ///          },
    ///         Err(err) => { println!("[Error] {err}") }
    ///     }   
    /// }
    /// ```
    pub async fn text(self, phone: &str, message: &str) -> Result<Value> {
        let url = format!("{}/text", self.endpoint);
        // Construct the JSON payload
        let mut payload = serde_json::Map::new();
        payload.insert("phone".into(),   Value::String(phone.to_owned()));
        payload.insert("message".into(), Value::String(message.to_owned()));
        payload.insert("key".into(),     Value::String(self.api_key.to_owned()));
        payload.insert("sender".into(), Value::String(self.sender.to_owned()));
        // Send the request
        let client = Client::new()
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        Ok(client.json().await?)
    }
}