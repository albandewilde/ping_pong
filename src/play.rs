use rand::Rng;

use serenity::model::channel::Message;

use crate::{helpers, score};

fn there_in_a_winner() -> bool {
    return rand::thread_rng().gen_range(0, 5) == 0;
}

fn the_bot_win() -> bool {
    return rand::thread_rng().gen_range(0, 3) == 0;
}

pub fn ping_pong(msg: &Message) -> String {
    let msg_content: String = helpers::format_msg(&msg.content);
    let username = helpers::discord_username(&msg.author);

    String::from(
        if (msg_content == "ping" || msg_content == "pong") && there_in_a_winner() {
            if the_bot_win() {
                score::update_score(&username, false);
                "You miss the ball.\nI won !"
            } else {
                score::update_score(&username, true);
                "I miss the ball.\nYou won, congrats."
            }
        } else {
            if msg_content == "ping" {
                "pong"
            } else if msg_content == "pong" {
                "ping"
            } else {
                "..."
            }
        },
    )
}
