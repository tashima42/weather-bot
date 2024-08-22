use reqwest::{Client, Error};
use serde_json::json;


pub async fn send_message(token: String, message: String, channel_id: String) -> Result<(), Error> {
    let message_body = json!({ "content": message });
    let discord_base_url: String = "https://discord.com/api/v10".into(); 
    let url = format!("{}/channels/{}/messages", discord_base_url, channel_id); 
    println!("URL: {}", url);

    let response = Client::new()
        .post(url)
        .header("authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .json(&message_body)
        .send()
        .await?
        .error_for_status()?;
    
    let status_code = response.status();
    println!("StatusCode: {}", status_code);

    Ok(())
}
