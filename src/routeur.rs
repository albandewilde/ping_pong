use serenity::{
    model::{channel::Message},
};

use crate::{play, score, helpers};

pub fn route(msg: &Message) -> String {
    let msg_content = msg.content.get(2..).unwrap().to_lowercase();

    if msg_content == "ping" || msg_content == "pong" {
        play::ping_pong(&msg)
    } else if msg_content == "score" {
        score::formated_score(&helpers::discord_username(&msg.author))
    } else {
        "Unknown command.".to_string()
    }
}
