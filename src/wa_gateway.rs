use reqwest::{Client, Error};
use serde_json::Value;

pub async fn send_wa_notification(api_url: &str, payload: Value) -> Result<(), Error> {
    let client = Client::new();
    let response = client
        .post(api_url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Notification sent successfully!");
    } else {
        println!(
            "Failed to send notification. Status: {:?}",
            response.status()
        );
    }
    Ok(())
}
