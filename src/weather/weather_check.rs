use std::env;

use reqwest;
use serde::Deserialize;
use teloxide::{
    prelude::*, types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup}
};
use once_cell::sync::Lazy;

static _API_KEY: Lazy<String> = Lazy::new(|| {
    env::var("METEOSOURCE_API_KEY")
    .expect("Meteosource API key not found")
});

#[derive(Deserialize)]
struct Location {
    #[serde(rename = "name")]
    id: String,
    #[serde(rename = "place_id")]
    name: String,
    #[serde(rename = "country")]
    country: String,
}

#[derive(Deserialize)]
struct Forecast {
    #[serde(rename = "summary")]
    forecast: String,
    #[serde(rename = "cloud_cover")]
    clouds: u32,
}



pub async fn search_place(query :&str) -> Result<Vec<Location>, reqwest::Error> {

    let url = format!("https://www.meteosource.com/api/v1/free/find_places?text={}=&key={}", query, _API_KEY.as_str());
    let resp = reqwest::get(&url).await?;
    let places : Vec<Location> = resp.json().await?;
    Ok(places)
}

pub async fn get_forecast(place : &str) -> Result<Forecast, reqwest::Error> {

    let url = format!("https://www.meteosource.com/api/v1/free/point?place_id={}=all&timezone=UTC&language=en&units=metric&key={}", place, _API_KEY.as_str());
    let resp = reqwest::get(&url).await?;
    let forecast : Forecast = resp.json().await?;
    Ok(forecast)
}

pub async fn search_handler(bot: Bot, msg: Message, query :&str) -> ResponseResult<()>{

    match search_place(query).await {
        Ok(locations) if !locations.is_empty() => {
            let buttons : Vec<Vec<InlineKeyboardButton>> = locations
            .iter()
            .map(|location| {
                vec! [InlineKeyboardButton::callback(
                    format!("{} - {}", location.name, location.country).clone(),
                    location.id.clone(),
                )]
            })
            .collect();

        let keyboard = InlineKeyboardMarkup::new(buttons);

        bot.send_message(msg.chat.id, "Select a location")
        .reply_markup(keyboard)
        .await?;
    
        Ok(())
        }

        Ok(_) | Err (_) => Ok(()),
    }
}

pub async fn forecast_callback_handler(bot: Bot, cb : CallbackQuery) -> ResponseResult<()> {

    let place_id = match cb.data.as_ref(){
        Some(place_id) => place_id,
        None => return Ok(())
    };

    let response = match get_forecast(&place_id).await {
        Ok(forecast) => format!("{} - {}", forecast.forecast, forecast.clouds),
        Err(err) => format!("Error fetching forecast for {}: {}", place_id, err),
    };

    bot.send_message(cb.chat_instance, &response).await?;
    bot.answer_callback_query(cb.id).await?;
    Ok(())
}