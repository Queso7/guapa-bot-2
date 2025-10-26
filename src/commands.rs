use serenity::prelude::*;

use serenity::model::channel::Message;

pub async fn handle_command(ctx: &Context, msg: &Message) {
    match msg.content.as_str() {
        "!ping" => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        "!hello" => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello there!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        _ => {}
    }
}