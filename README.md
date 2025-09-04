<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="img/textbelt-rs-transparent-dark-theme.png">
        <source media="(prefers-color-scheme: light)" srcset="img/textbelt-rs-transparent-light-theme.png">
        <img src="img/textbelt-rs-transparent-dark-theme.png" alt="textbelt-rs logo" width='400' />
    </picture>
</p>

<hr />

`textbelt-rs` is a Rust library for [textbelt](https://textbelt.com/). :crab:

Textbelt is an SMS API that is built for developers who just want to send and receive SMS. Sending an SMS is a simple thing. The goal is to provide an API that is correspondingly simple, without requiring account configuration, logins, or extra recurring billing. You shouldn’t send more than 1–2 SMS per second; this is the rate limit imposed by the [official service](https://docs.textbelt.com/faq/sending-and-receiving-messages?q=change+from+name#is-there-any-rate-limiting). Please read the [official documentation](https://docs.textbelt.com/) for more information.

- [CHANGELOG.md](CHANGELOG.md) - A record of all significant version changes
- [examples/](examples/) - Library usage example in real Rust project

## Implemented Features

| Method | Resource                    | Description                        |
|:------:|:----------------------------|:-----------------------------------|
| POST  | [/text](https://docs.textbelt.com/#send-an-sms-using-http-post)                     | Can be use to send message to a phone number (a normal 10-digit phone number with area code). |
| GET   | [/status/:textId](https://docs.textbelt.com/other-api-endpoints#checking-sms-delivery-status)           | If you are given a **textId** and want to check its delivery status. |
| GET   | [/quota/:key](https://docs.textbelt.com/other-api-endpoints#checking-your-credit-balance)               | You may want to know how much quota or credit you have left on a **key**. |

## Example

**Cargo.toml**:
```rust
...
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
textbelt = { version = "1.0.0" }
```

**main.rs**:

> How to send an text SMS?

```rust
use textbelt::TextbeltClient;

#[tokio::main]
async fn main() {
    let tc = TextbeltClient::new("Your textbelt API Key");
    let phone = "+33601020304";
    let message = "Hello from textbelt-rs!";
    match tc.text(&phone, &message).await {
        Ok(res) => {
            println!("{:?}", &res);
         },
        Err(err) => { println!("[Error] {err}") }
    }   
}
```

> How to check the delivery status?

```rust
use textbelt::TextbeltClient;

#[tokio::main]
async fn main() {
    let tc = TextbeltClient::new("Your textbelt API Key");
    let text_id = "text_id";
    match tc.status(&text_id).await {
        Ok(res) => {
            println!("{:?}", &res);
         },
        Err(err) => { println!("[Error] {err}") }
    }   
}
```

> How to check the api key quota?

```rust
use textbelt::TextbeltClient;

#[tokio::main]
async fn main() {
    let tc = TextbeltClient::new("Your textbelt API Key");
    match tc.quota().await {
        Ok(res) => {
            println!("{:?}", &res);
         },
        Err(err) => { println!("[Error] {err}") }
    }
}
```

More examples in [doc.rs/textbelt](https://docs.rs/textbelt)

# Special thanks to 

[![](https://github.com/branoodle.png?size=50)](https://github.com/branoodle)
