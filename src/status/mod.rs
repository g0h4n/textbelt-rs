use super::{TextbeltClient, Value};
use anyhow::Result;
use reqwest::Client;

impl <'a> TextbeltClient<'a> {
    /// # How to check the delivery status?
    /// <https://docs.textbelt.com/other-api-endpoints#checking-sms-delivery-status>
    ///
    /// # Example
    ///
    /// ```rust
    /// use textbelt::TextbeltClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let tc = TextbeltClient::new("Your textbelt API Key", None);
    ///     let text_id = "text_id";
    ///     match tc.status(&text_id).await {
    ///         Ok(res) => {
    ///             println!("{:?}", &res);
    ///          },
    ///         Err(err) => { println!("[Error] {err}") }
    ///     }   
    /// }
    /// ```
    pub async fn status(self, text_id: &str) -> Result<Value> {
        let url = format!("{}/status/{text_id}",self.endpoint);
        let client = Client::new()
            .get(&url)
            .send()
            .await?;
        Ok(client.json().await?)
    }
}