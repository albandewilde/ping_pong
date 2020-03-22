use rand::Rng;

fn there_in_a_winner() -> bool {
    return rand::thread_rng().gen_range(0, 5) == 0;
}

fn the_bot_win() -> bool {
    return rand::thread_rng().gen_range(0, 3) == 0;
}

fn results(ask: &str) -> String {
    let pp = ask.to_lowercase();
    if there_in_a_winner() && (pp == "ping" || pp == "pong") {
        let mut win = String::from("There is a winner.\n");
        if the_bot_win() {
            win.push_str("I won !");
        } else {
            win.push_str("You won, congrats.");
        }
        win
    } else {
        String::from(
            if pp == "ping" {
                "pong"
            } else if pp == "pong" {
                "ping"
            } else {
                "..."
            }
        )
    }
}

fn main() {
    println!("{}", results("plop"));
}
