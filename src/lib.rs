//! <p align="center">
//! <img width="60%" src="https://textbelt.com/images/logo_black_transparent.png">
//! </p>
//!
//!
//! Official API documentation: <https://docs.textbelt.com/>
//!
//! Use "**TextbeltClient**" struct.
//!
//! # Example
//!
//! ```
//! use textbelt::*;
//!
//! let tc = TextbeltClient::new("Your API Key");
//! let phone_number = "+330601020304";
//! let message = "Hello from textbelt-rs API!";
//! tc.send_sms(&phone_number, &message).await?;
//! ```
//!

use serde::Deserialize;
use serde_json::Value;

// FIXME