use rand::Rng;
use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

fn there_in_a_winner() -> bool {
    return rand::thread_rng().gen_range(0, 5) == 0;
}

fn the_bot_win() -> bool {
    return rand::thread_rng().gen_range(0, 3) == 0;
}

fn results(ask: &str) -> String {
    let pp = ask.to_lowercase();
    String::from(
        if there_in_a_winner() && (pp == "ping" || pp == "pong") {
            if the_bot_win() {
                "You miss the ball.\nI won !"
            } else {
                "I miss the ball.\nYou won, congrats."
            }
        } else {
            if pp == "ping" {
                "pong"
            } else if pp == "pong" {
                "ping"
            } else {
                "..."
            }
        }
    )
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.chars().nth(0).unwrap() == '¨' {

            // 2 because ¨ is a two bites caracter
            let result = results(&msg.content.get(2..).unwrap());

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
