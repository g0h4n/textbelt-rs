use anyhow::Result;
use serde_json::Value;
use textbelt::TextbeltClient;

#[tokio::main]
async fn main() -> Result<()> {

    // Create textbelt client
    let tc = TextbeltClient::new("APIKEY", Some("Noodle/g0h4n"));
    // Check API Key quota
    let quota = tc.quota().await?;
    println!("[?] Quota:{:?}", quota);

    // Phone number to change
    let phone = "+33601020304";
    // Message to change
    let msg = "Hello from textbelt-rs!\nNoodle/g0h4n";
    // Send message
    let check = tc.text(phone, msg).await?;

    // Check status
    if let Some(success) = check.get("success").and_then(Value::as_bool) {
        if success {
            let text_id = check.get("textId").and_then(Value::as_str).unwrap_or("N/A");
            let msg_status = tc.status(text_id).await?;
            println!("[+] Message sent successfully!\n{:?}", msg_status);
        } else {
            if let Some(err) = check.get("error").and_then(Value::as_str) {
                let quota = tc.quota().await?;
                println!("[!] Error: {}",err);
                println!("[?] Remaining quota: {:?}", quota);
            }
        }
    } else {
        println!("[!] Unexcepted response format. No error message found");
    }
    Ok(())

}