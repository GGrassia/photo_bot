use crate::weather_data_model::weather_data::*;
use teloxide::{prelude::*, types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup}};
use reqwest;
use serde::Deserialize;
use serde_json;
use std::env;
use once_cell::sync::Lazy;
use crate::base::{to_error, State};
use crate::{base, MyDialogue};
use crate::HandlerResult;

static API_KEY: Lazy<String> = Lazy::new(|| {
    env::var("METEOSOURCE_API_KEY")
        .expect("Meteosource API key not found")
});

#[derive(Deserialize)]
struct Location {
    name: String,
    place_id: String,
    country: String,
}

async fn search_place(query: &str) -> Result<Vec<Location>, reqwest::Error> {
    let url = format!("https://www.meteosource.com/api/v1/free/find_places?text={}&key={}",
                      query, API_KEY.as_str());
    let resp = reqwest::get(&url).await?;
    let places: Vec<Location> = resp.json().await?;
    Ok(places)
}

async fn get_forecast(place: &str) ->  Result<WeatherForecast, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://www.meteosource.com/api/v1/free/point?place_id={}&sections=all&timezone=UTC&language=en&units=metric&key={}",
                      place, API_KEY.as_str());
    let resp = reqwest::get(&url).await?;
    let text = resp.text().await?;
    log::info!("Raw Response: {}", text);
    let forecast: WeatherForecast = serde_json::from_str(&text)?;
    Ok(forecast)
}

pub async fn handle(bot: Bot, dialogue: MyDialogue, msg: Message, query: String) -> HandlerResult {
    if query.is_empty() {
        let _ = base::to_error(bot.send_message(
            msg.chat.id,
            "Please provide a location. Example: /weather London"
        ).await)?;
        return Ok(());
    }

    match search_place(&query).await {
        Ok(locations) if !locations.is_empty() => {
            let buttons: Vec<Vec<InlineKeyboardButton>> = locations
                .iter()
                .map(|location| {
                    vec![InlineKeyboardButton::callback(
                        format!("{} - {}", location.name, location.country),
                        location.place_id.clone(),
                    )]
                })
                .collect();

            let keyboard = InlineKeyboardMarkup::new(buttons);

            let _ = base::to_error(bot.send_message(msg.chat.id, "Select a location:")
                .reply_markup(keyboard)
                .await)?;

            log::info!("About to change state");

            dialogue.update(State::AwaitingLocationSelection { query }).await.expect("TODO: panic message");

            log::info!("state changed");
        },
        Ok(_) => {
            let _ = base::to_error(bot.send_message(
                msg.chat.id,
                format!("No locations found for '{}'", query),
            ).await)?;

        },
        Err(e) => {
            let _ = base::to_error(bot.send_message(
                msg.chat.id,
                format!("Error searching for locations: {}", e),
            ).await)?;
        }
    }

    Ok(())
}

pub async fn handle_callback(bot: Bot, dialogue: MyDialogue, query: String, cb: CallbackQuery) -> HandlerResult {
    println!("Got callback query: {:?}", cb);
    log::info!("Got callback query: {:?}", cb);
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

                        let daily_forecast = format!(
                            "Daily Forecast: \nSummary for {}: {} \nPrecipitations: {}\nCloud Cover:{}",
                            forecast.daily.date,
                            forecast.daily.summary,
                            forecast.daily.precipitation,
                            forecast.daily.cloud_cover,
                       );
                        let _ = base::to_error(bot.send_message(chat_id, daily_forecast).await)?;
                },
                Err(e) => {
                    let _ = base::to_error(bot.send_message(
                        chat_id,
                        format!("Error fetching forecast: {}", e),
                    ).await)?;
                }
            }
        }
        dialogue.exit().await?;
    }

    Ok(())
}