use crate::{
    messages::ServerMessage,
    player::Player,
    questions::{Question, battle_question_count, question_at},
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct MatchState {
    pub players: [Player; 2],
    start_question: usize,
    question_number: usize,
    answers: HashMap<Uuid, String>,
    scores: HashMap<Uuid, usize>,
}

impl MatchState {
    pub fn new(players: [Player; 2], start_question: usize) -> Self {
        Self {
            players,
            start_question,
            question_number: 1,
            answers: HashMap::new(),
            scores: HashMap::new(),
        }
    }

    pub fn current_question(&self) -> Question {
        question_at(self.start_question + self.question_number - 1)
    }

    pub fn question_index(&self) -> usize {
        self.start_question + self.question_number - 1
    }

    pub fn question_number(&self) -> usize {
        self.question_number
    }

    pub fn has_bot(&self) -> bool {
        self.players.iter().any(|player| player.is_bot)
    }

    pub fn bot_id(&self) -> Option<Uuid> {
        self.players
            .iter()
            .find(|player| player.is_bot)
            .map(|player| player.id)
    }

    pub fn add_answer(&mut self, player_id: Uuid, answer: String) -> bool {
        self.answers.entry(player_id).or_insert(answer);

        if self.answers.len() < 2 {
            return false;
        }

        self.score_answers();
        self.answers.clear();
        self.question_number += 1;
        true
    }

    pub fn is_done(&self) -> bool {
        self.question_number > battle_question_count()
    }

    pub fn send_next_question(&self) {
        self.send_to_both(ServerMessage::Question {
            number: self.question_number,
            question: self.current_question(),
        });
    }

    pub fn send_game_over(&self) {
        for player in &self.players {
            player.send(ServerMessage::GameOver {
                score: self.score(player.id),
                opponent_score: self.opponent_score(player.id),
            });
        }
    }

    fn score_answers(&mut self) {
        let question = self.current_question();
        for (player_id, answer) in &self.answers {
            if answer == &question.answer {
                *self.scores.entry(*player_id).or_default() += 1;
            }
        }
    }

    fn send_to_both(&self, message: ServerMessage) {
        for player in &self.players {
            player.send(message.clone());
        }
    }

    fn score(&self, player_id: Uuid) -> usize {
        self.scores.get(&player_id).copied().unwrap_or(0)
    }

    fn opponent_score(&self, player_id: Uuid) -> usize {
        self.players
            .iter()
            .find(|player| player.id != player_id)
            .map(|opponent| self.score(opponent.id))
            .unwrap_or(0)
    }
}
