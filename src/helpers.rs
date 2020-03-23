use rand::Rng;

fn there_in_a_winner() -> bool {
    return rand::thread_rng().gen_range(0, 5) == 0;
}

fn the_bot_win() -> bool {
    return rand::thread_rng().gen_range(0, 3) == 0;
}

pub fn results(ask: &str) -> String {
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
