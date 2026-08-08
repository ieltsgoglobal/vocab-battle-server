use crate::questions::Question;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "answer")]
    Answer { answer: String },
}

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "matched")]
    Matched {
        opponent_id: String,
        number: usize,
        question: Question,
    },
    #[serde(rename = "question")]
    Question { number: usize, question: Question },
    #[serde(rename = "game_over")]
    GameOver { score: usize, opponent_score: usize },
    #[serde(rename = "opponent_left")]
    OpponentLeft,
}
