use std::time::Instant;

use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let now = Instant::now();
    let _ = msg.channel_id.say(&ctx.http, "Pinging bot ..").await?;

    let latency = now.elapsed().as_millis();

    msg.channel_id.say(&ctx.http, format!("Latency: {} ms", latency)).await?;
    Ok(())
}

