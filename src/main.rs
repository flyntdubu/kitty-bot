extern crate dotenv;

use dotenv::dotenv;
use reqwest::Url;
use std::error::Error;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv().ok();

    log::info!("Starting...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "gives a kitty.")]
    Meow,
}

async fn answer(bot: Bot, m: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Meow => {
            let chat = &m.chat;

            if let Some(maybe_url) = m.text() {
                if maybe_url == "/meow" || maybe_url == "meow" {
                    let response =
                        reqwest::get("https://api.thecatapi.com/v1/images/search").await?;

                    if response.status().is_success() {
                        // Parse the JSON response
                        let cat_data: Vec<serde_json::Value> = response.json().await?;

                        // Extract the URL of the cat image
                        if let Some(url) = cat_data[0].get("url") {
                            let p = Url::parse(url.as_str().unwrap()).unwrap();
                            bot.send_photo(chat.id, InputFile::url(p)).await?;
                        } else {
                            bot.send_message(
                                chat.id,
                                "The kitty services are down ): Try again later.",
                            )
                            .await?;
                        }
                    } else {
                        bot.send_message(
                            chat.id,
                            "The kitty services are down ): Try again later.",
                        )
                        .await?;
                    }
                }
            }
        }
    };

    Ok(())
}
