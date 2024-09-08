mod commands;
mod handler;
mod chat;

// use chat::openai_prompt::OpenAI;
use handler::Handler;
use serenity::{framework::StandardFramework, prelude::*};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("DISCORD_TOKEN must be set");
    // let openai = OpenAI::new();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("potofu"));

    let mut client = Client::builder(&token,GatewayIntents::all())
        .event_handler(Handler{ })
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
