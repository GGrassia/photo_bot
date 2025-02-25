use teloxide::{prelude::*, types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup}};
use reqwest;
use serde::Deserialize;
use std::env;
use once_cell::sync::Lazy;

static API_KEY: Lazy<String> = Lazy::new(|| {
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

async fn search_place(query: &str) -> Result<Vec<Location>, reqwest::Error> {
    let url = format!("https://www.meteosource.com/api/v1/free/find_places?text={}&key={}",
                      query, API_KEY.as_str());
    let resp = reqwest::get(&url).await?;
    let places: Vec<Location> = resp.json().await?;
    Ok(places)
}

async fn get_forecast(place: &str) -> Result<Forecast, reqwest::Error> {
    let url = format!("https://www.meteosource.com/api/v1/free/point?place_id={}&sections=all&timezone=UTC&language=en&units=metric&key={}",
                      place, API_KEY.as_str());
    let resp = reqwest::get(&url).await?;
    let forecast: Forecast = resp.json().await?;
    Ok(forecast)
}

pub async fn handle(bot: Bot, msg: Message, query: String) -> ResponseResult<()> {
    if query.is_empty() {
        bot.send_message(
            msg.chat.id,
            "Please provide a location. Example: /weather London"
        ).await?;
        return Ok(());
    }

    match search_place(&query).await {
        Ok(locations) if !locations.is_empty() => {
            let buttons: Vec<Vec<InlineKeyboardButton>> = locations
                .iter()
                .map(|location| {
                    vec![InlineKeyboardButton::callback(
                        format!("{} - {}", location.name, location.country),
                        location.id.clone(),
                    )]
                })
                .collect();

            let keyboard = InlineKeyboardMarkup::new(buttons);

            bot.send_message(msg.chat.id, "Select a location:")
                .reply_markup(keyboard)
                .await?;
        },
        Ok(_) => {
            bot.send_message(
                msg.chat.id,
                format!("No locations found for '{}'", query),
            ).await?;
        },
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("Error searching for locations: {}", e),
            ).await?;
        }
    }

    Ok(())
}

pub async fn handle_callback(bot: Bot, cb: CallbackQuery) -> ResponseResult<()> {
    if let Some(place_id) = &cb.data {
        // Answer the callback to remove the loading indicator
        bot.answer_callback_query(&cb.id).await?;

        // Get chat ID from the message
        if let Some(msg) = cb.message {
            let chat_id = msg.chat().id;

            // Notify user that we're fetching the forecast
            bot.send_message(chat_id, "Fetching forecast...").await?;

            // Get the forecast
            match get_forecast(place_id).await {
                Ok(forecast) => {
                    bot.send_message(
                        chat_id,
                        format!("Weather: {}\nCloud cover: {}%",
                                forecast.forecast,
                                forecast.clouds),
                    ).await?;
                },
                Err(e) => {
                    bot.send_message(
                        chat_id,
                        format!("Error fetching forecast: {}", e),
                    ).await?;
                }
            }
        }
    }

    Ok(())
}