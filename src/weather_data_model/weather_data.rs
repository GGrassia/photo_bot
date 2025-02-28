use serde::Deserialize;
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub daily: DailyContainer,
    pub hourly: HourlyData,
}

#[derive(Debug, Deserialize)]
pub struct DailyContainer {
    pub data: Vec<DailyData>,
    #[serde(rename = "type")]
    pub forecast_type: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyData {
    pub day: NaiveDate,
    pub summary: String,
    #[serde(rename = "all_day")]
    pub all_day: AllDayData,
}

#[derive(Debug, Deserialize)]
pub struct AllDayData {
    pub weather: String,
    pub temperature: f64,
    pub temperature_min: f64,
    pub temperature_max: f64,
    pub wind: Wind,
    pub cloud_cover: CloudCover,
    pub precipitation: Precipitation,
}

#[derive(Debug, Deserialize)]
pub struct HourlyData {
    pub date: DateTime<Utc>,
    pub weather: String,
    pub summary: String,
    pub temperature: f64,
    pub wind: Wind,
    pub cloud_cover: CloudCover,
    pub precipitation: Precipitation,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub dir: String,
    pub angle: u16,
}

#[derive(Debug, Deserialize)]
pub struct CloudCover {
    pub total: u8,
}

#[derive(Debug, Deserialize)]
pub struct Precipitation {
    pub total: f64,
    #[serde(rename = "type", default = "default_precipitation_type")]
    pub precipitation_type: String,
}

fn default_precipitation_type() -> String {
    "none".to_string()
}

// Create your custom model for your application needs
#[derive(Debug, Deserialize)]
pub struct WeatherForecast {
    pub daily: DailyForecast,
    pub hourly: Vec<HourlyForecast>,
}

#[derive(Debug, Deserialize)]
pub struct DailyForecast {
    pub date: NaiveDate,
    pub summary: String,
    pub cloud_cover: u8,
    pub precipitation: f64,
}

#[derive(Debug, Deserialize)]
pub struct HourlyForecast {
    pub hour: DateTime<Utc>,
    pub cloud_cover: u8,
    pub precipitation: f64,
}