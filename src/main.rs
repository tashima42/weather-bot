pub mod weather;
pub mod discord;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let hg_key = std::env::var("HG_KEY").unwrap();
  let discord_token = std::env::var("DISCORD_TOKEN").unwrap();
  let channel_id = std::env::var("CHANNEL_ID").unwrap();
  let w = weather::request_weather(hg_key).await?;
  let message = format!("**-------Weather-------**
* City: {}
* Date: {}
* Temperature: {}°C - {}°C
* Humidity: {}%
* Rain: {} mm
* Rain Probability: {}%",
      w.results.city,
      w.results.forecast.get(0).unwrap().date,
      w.results.forecast.get(0).unwrap().min,
      w.results.forecast.get(0).unwrap().max,
      w.results.forecast.get(0).unwrap().humidity,
      w.results.forecast.get(0).unwrap().rain,
      w.results.forecast.get(0).unwrap().rain_probability,
    );
  discord::send_message(discord_token, message, channel_id).await?;
  Ok(())
}
