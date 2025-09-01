<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://textbelt.com/images/logo_black_transparent.png">
        <source media="(prefers-color-scheme: light)" srcset="https://textbelt.com/images/logo_black_transparent.png">
        <img src="https://textbelt.com/images/logo_black_transparent.png" alt="rusthound-ce logo" width='250' />
    </picture>
</p>

<hr />

Library for [textbelt](https://textbelt.com/) written in Rust. :crab:
FIXME

## Implemented Features

| Method | Resource                    | Description                        |
|:------:|:----------------------------|:-----------------------------------|
| POST  | /FIXME                    | FIXME                       |
| GET   | /FIXME                    | FIXME                       |
| GET   | /FIXME                    | FIXME                       |

## Example

**Cargo.toml**:
```rust
...
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
textbelt = { version = "1.0.0" }
```

**main.rs**:

```rust
use textbelt::TextbeltClient;

#[tokio::main]
async fn main() {
    let api = "Your API KEY";
    let vt = TextbeltClient::new(api);
    match vt.send_sms(&phone_number, &message).await {
        Ok(res) => {
            println!("{:?}", &res.data);
         },
        Err(err) => { println!("[Error] {err}") }
    }   
}
```

More examples in [doc.rs/textbelt](https://docs.rs/textbelt)

# Special thanks to 

[![](https://github.com/branoodle.png?size=50)](https://github.com/branoodle)
