use serenity::model::channel::Message;

use crate::{helpers, play, score};

pub fn route(msg: &Message) -> String {
    let msg_content = helpers::format_msg(&msg.content);

    if msg_content == "ping" || msg_content == "pong" {
        play::ping_pong(&msg)
    } else if msg_content == "score" {
        score::formated_score(&helpers::discord_username(&msg.author))
    } else {
        "Unknown command.".to_string()
    }
}
