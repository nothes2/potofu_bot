use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
// use crate::chat::openai_prompt::OpenAI;
use crate::commands::admin;

pub struct Handler {
    // pub openai: OpenAI,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content == "potofu ping" {
            if let Err(why) = admin::ping(&ctx, &msg).await {
                println!("Error executing ping command:{:?}", why);
            }
        }

        // if msg.content.starts_with("potofu") {
        //     let prompt = msg.content.trim();
        //     match self.openai.generate_text(prompt).await {
        //         Ok(response) => {
        //             if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        //                 println!("Error sending message: {:?}", why);
        //             }
        //         },
        //         Err(err) => println!("Error generating text: {:?}", err),
        //     }
        // }
    }
}