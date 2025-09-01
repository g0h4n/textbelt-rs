//! <p align="center">
//!     <picture>
//!         <source media="(prefers-color-scheme: dark)" srcset="./img/textbelt-rs-transparent-dark-theme.png">
//!         <source media="(prefers-color-scheme: light)" srcset="./img/textbelt-rs-transparent-light-theme.png">
//!         <img src="./img/textbelt-rs-transparent-dark-theme.png" alt="textbelt-rs logo" width='250' />
//!     </picture>
//! </p>
//
//! <hr />
//! 
//! `textbelt-rs` is a Rust library for [textbelt](https://textbelt.com/). :crab:
//! 
//! Textbelt is an SMS API that is built for developers who just want to send and receive SMS. Sending an SMS is a simple thing. The goal is to provide an API that is correspondingly simple, without requiring account configuration, logins, or extra recurring billing. 
//!
//! Official API documentation: <https://docs.textbelt.com/>
//!
//! Use "**TextbeltClient**" struct.
//!
//! ```
//! use textbelt::*;
//!
//! let tc = TextbeltClient::new("Your textbelt API Key");
//! let phone = "+33601020304";
//! let message = "Hello from textbelt-rs API!";
//! tc.text(&phone, &message).await?;
//! ```
//! 
use serde::Deserialize;
use serde_json::Value;

/// # How to send an text SMS?
/// 
/// ```
/// use textbelt::*;
/// 
/// let tc = TextbeltClient::new("Your textbelt API Key");
/// let phone = "+33601020304";
/// let message = "Hello from textbelt-rs API!";
/// tc.text(&phone, &message).await?;
/// ```
pub mod text;

/// # How to check the delivery status?
/// 
/// ```rust
/// let tc = TextbeltClient::new("Your textbelt API Key");
/// let text_id = "text_id";
/// tc.status(&text_id).await?;
/// ```
pub mod status;

/// # How to check the delivery status?
///  
/// ```rust
/// let tc = TextbeltClient::new("Your textbelt API Key");
/// let key = "key";
/// tc.quota(&key).await?
/// ```
pub mod quota;

/// TextbeltClient structure
/// <https://docs.textbelt.com/>
#[derive(Copy, Clone)]
pub struct TextbeltClient<'a> {
    /// Your API key for access to Textbelt services
    api_key: &'a str,
}
impl<'a> TextbeltClient<'a> {
    pub fn new(api_key: &'a str) -> Self {
        TextbeltClient {
            api_key,
            endpoint: "https://textbelt.com/",
        }
    }
}