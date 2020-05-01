use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy)]
struct PlayerScore {
    player_score: usize,
    bot_score: usize
}

fn get_scores() -> HashMap<String, PlayerScore> {
    let mut file = File::open("/scores.json").unwrap();
    let mut scores_file = String::new();
    file.read_to_string(&mut scores_file).unwrap();

    let scores: HashMap<String, PlayerScore> = serde_json::from_str(&scores_file).unwrap();
    scores
}

fn get_score(user_name: &str) -> PlayerScore {
    let scores = get_scores();

    match scores.get(user_name) {
        None => PlayerScore {
            player_score: 0,
            bot_score: 0
        },
        Some(ps) => *ps
    }
}

fn write_scores(scores: HashMap<String, PlayerScore>) {
    let file = File::create("/scores.json").unwrap();
    serde_json::to_writer(file, &scores).unwrap();
}

pub fn formated_score(user_name: &str) -> String {
    let score = get_score(user_name);
    format!(
        "```\n{}: {}\nPPP: {}\n```",
        user_name,
        score.player_score,
        score.bot_score
    )
}

pub fn update_score(user_name: &str, user_win: bool) {
    let mut score = get_score(user_name);

    if user_win {
        score.player_score += 1;
    } else {
        score.bot_score += 1;
    }

    let mut scores = get_scores();
    scores.insert(user_name.to_string(), score);

    write_scores(scores);
}
