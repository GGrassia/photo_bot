use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub lat: String,
    pub lon: String,
    pub elevation: i32,
    pub timezone: String,
    pub units: String,
    pub current: CurrentWeather,
    pub hourly: HourlyForecast,
    pub daily: DailyForecast,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub icon: String,
    pub icon_num: i32,
    pub summary: String,
    pub temperature: f32,
    pub wind: Wind,
    pub precipitation: Precipitation,
    pub cloud_cover: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub angle: f32,
    pub dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Precipitation {
    pub total: f32,
    pub r#type: String,  // Using raw identifier for 'type' keyword
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyForecast {
    pub data: Vec<HourlyData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyData {
    pub date: String,
    pub weather: String,
    pub icon: i32,
    pub summary: String,
    pub temperature: f32,
    pub wind: HourlyWind,
    pub cloud_cover: CloudCover,
    pub precipitation: Precipitation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyWind {
    pub speed: f32,
    pub dir: String,
    pub angle: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudCover {
    pub total: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyForecast {
    pub data: Vec<DailyData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyData {
    pub day: String,
    pub weather: String,
    pub icon: i32,
    pub summary: String,
    pub all_day: AllDayWeather,
    pub morning: Option<DayTimeWeather>,
    pub afternoon: Option<DayTimeWeather>,
    pub evening: Option<DayTimeWeather>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllDayWeather {
    pub weather: String,
    pub icon: i32,
    pub temperature: f32,
    pub temperature_min: f32,
    pub temperature_max: f32,
    pub wind: Wind,
    pub cloud_cover: CloudCover,
    pub precipitation: Precipitation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayTimeWeather {
    // The fields for this struct are not visible in the sample data
    // as all values are null. Adding placeholder fields that can be
    // updated when actual data is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<Wind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation: Option<Precipitation>,
}