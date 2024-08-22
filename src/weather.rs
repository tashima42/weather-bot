use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use reqwest::Error;

// Curitiba
const WOEID: u32 = 455822;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub by: Value,
    #[serde(rename = "valid_key")]
    pub valid_key: bool,
    pub results: Results,
    #[serde(rename = "execution_time")]
    pub execution_time: f64,
    #[serde(rename = "from_cache")]
    pub from_cache: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    pub temp: i64,
    pub date: String,
    pub time: String,
    #[serde(rename = "condition_code")]
    pub condition_code: String,
    pub description: String,
    pub currently: String,
    pub cid: String,
    pub city: String,
    #[serde(rename = "img_id")]
    pub img_id: String,
    pub humidity: i64,
    pub cloudiness: f64,
    pub rain: f64,
    #[serde(rename = "wind_speedy")]
    pub wind_speedy: String,
    #[serde(rename = "wind_direction")]
    pub wind_direction: i64,
    #[serde(rename = "wind_cardinal")]
    pub wind_cardinal: String,
    pub sunrise: String,
    pub sunset: String,
    #[serde(rename = "moon_phase")]
    pub moon_phase: String,
    #[serde(rename = "condition_slug")]
    pub condition_slug: String,
    #[serde(rename = "city_name")]
    pub city_name: String,
    pub timezone: String,
    pub forecast: Vec<Forecast>,
    pub cref: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub date: String,
    pub weekday: String,
    pub max: i64,
    pub min: i64,
    pub humidity: i64,
    pub cloudiness: f64,
    pub rain: f64,
    #[serde(rename = "rain_probability")]
    pub rain_probability: i64,
    #[serde(rename = "wind_speedy")]
    pub wind_speedy: String,
    pub sunrise: String,
    pub sunset: String,
    #[serde(rename = "moon_phase")]
    pub moon_phase: String,
    pub description: String,
    pub condition: String,
}

pub async fn request_weather(hg_key: String) -> Result<Weather, Error> {
  let url = format!("https://api.hgbrasil.com/weather?woeid={}&format=json&key={}", WOEID, hg_key); 
  println!("URL: {}", url);
  let response = reqwest::get(&url).await?;
  let weather: Weather = response.json().await?;
  Ok(weather)
}
