use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub mod helpers;
pub mod play;
mod routeur;
pub mod score;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // Check the prefix of the message
        let prefix = match msg.content.chars().nth(0) {
            Some(chr) => chr,
            None => '0',
        };

        // Check if it is our prefix
        if prefix == '¨' {
            let resp = routeur::route(&msg);

            if let Err(why) = msg.channel_id.say(&ctx.http, resp) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready to match !", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start_shards(2) {
        println!("Client error: {:?}", why);
    }
}
