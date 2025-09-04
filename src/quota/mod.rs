use super::{TextbeltClient,Value};
use anyhow::Result;
use reqwest::Client;

impl<'a> TextbeltClient<'a> {
    /// # How to check the api key quota?
    /// <https://docs.textbelt.com/other-api-endpoints#checking-your-credit-balance>
    ///
    /// # Example
    ///
    /// ```rust
    /// use textbelt::TextbeltClient;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let tc = TextbeltClient::new("Your textbelt API Key", None);
    ///     match tc.quota().await {
    ///         Ok(res) => {
    ///             println!("{:?}", &res);
    ///          },
    ///         Err(err) => { println!("[Error] {err}") }
    ///     }
    /// }
    /// ```
    pub async fn quota(self) -> Result<Value> {
        let url = format!("{}/quota/{}",self.endpoint,self.api_key);
        let client = Client::new()
            .get(&url)
            .send()
            .await?;
        Ok(client.json().await?)
    }
}