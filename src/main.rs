use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod helpers;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.chars().nth(0).unwrap() == '¨' {

            // 2 because ¨ is a two bites caracter
            let result = helpers::results(&msg.content.get(2..).unwrap());

            if let Err(why) = msg.channel_id.say(&ctx.http, result) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready to match !", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start_shards(2) {
        println!("Client error: {:?}", why);
    }
}
