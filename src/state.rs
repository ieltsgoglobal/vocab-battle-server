use crate::{
    match_state::MatchState,
    messages::ServerMessage,
    player::Player,
    questions::{question_at, question_count},
};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::Mutex;
use uuid::Uuid;

pub type SharedState = Arc<Mutex<AppState>>;

pub struct AppState {
    waiting: VecDeque<Player>,
    matches: HashMap<Uuid, MatchState>,
    player_rooms: HashMap<Uuid, Uuid>,
    next_question: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            waiting: VecDeque::new(),
            matches: HashMap::new(),
            player_rooms: HashMap::new(),
            next_question: 0,
        }
    }

    pub fn join(&mut self, player: Player) -> bool {
        let Some(other) = self.waiting.pop_front() else {
            player.send(ServerMessage::Waiting);
            self.waiting.push_back(player);
            println!("waiting={}", self.waiting.len());
            return true;
        };

        self.start_match(other, player);
        false
    }

    pub fn match_with_bot_if_waiting(&mut self, player_id: Uuid) -> bool {
        let Some(index) = self
            .waiting
            .iter()
            .position(|player| player.id == player_id)
        else {
            return false;
        };

        let player = self.waiting.remove(index).unwrap();
        let (tx, _) = tokio::sync::mpsc::unbounded_channel();
        let bot = Player {
            id: Uuid::new_v4(),
            tx,
            is_bot: true,
        };
        self.start_match(player, bot);
        true
    }

    pub fn answer_bot(&mut self, human_id: Uuid, random: fn(usize) -> usize) {
        let Some(room_id) = self.player_rooms.get(&human_id).copied() else {
            return;
        };
        let Some(match_state) = self.matches.get(&room_id) else {
            return;
        };
        let Some(bot_id) = match_state.bot_id() else {
            return;
        };

        let question = question_at(match_state.question_index());
        let answer = question.options[random(question.options.len())].to_string();
        self.answer(bot_id, answer);
    }

    fn start_match(&mut self, first: Player, second: Player) {
        let room_id = first.id;
        let match_state = MatchState::new([first.clone(), second.clone()], self.next_question);
        let first_question = match_state.current_question();
        self.next_question += 1;

        self.player_rooms.insert(first.id, room_id);
        self.player_rooms.insert(second.id, room_id);
        self.matches.insert(room_id, match_state);

        first.send(ServerMessage::Matched {
            opponent_id: second.id.to_string(),
            number: 1,
            question: first_question.clone(),
        });
        second.send(ServerMessage::Matched {
            opponent_id: first.id.to_string(),
            number: 1,
            question: first_question,
        });

        println!(
            "matched {} vs {}; waiting={}",
            first.id,
            second.id,
            self.waiting.len()
        );
    }

    pub fn answer(&mut self, player_id: Uuid, answer: String) {
        let Some(room_id) = self.player_rooms.get(&player_id).copied() else {
            return;
        };

        let finished_players = {
            let Some(match_state) = self.matches.get_mut(&room_id) else {
                return;
            };

            if !match_state.add_answer(player_id, answer) {
                return;
            }

            if match_state.is_done() {
                match_state.send_game_over();
                Some(match_state.players.clone())
            } else {
                match_state.send_next_question();
                None
            }
        };

        if let Some(players) = finished_players {
            self.matches.remove(&room_id);
            for player in players {
                self.player_rooms.remove(&player.id);
            }
        }
    }

    pub fn leave(&mut self, player_id: Uuid) {
        self.waiting.retain(|player| player.id != player_id);

        if let Some(room_id) = self.player_rooms.remove(&player_id) {
            if let Some(match_state) = self.matches.remove(&room_id) {
                for player in match_state.players {
                    self.player_rooms.remove(&player.id);
                    if player.id != player_id {
                        player.send(ServerMessage::OpponentLeft);
                    }
                }
            }
        }

        println!("{player_id} left; waiting={}", self.waiting.len());
    }

    pub fn should_bot_answer(&self, player_id: Uuid) -> bool {
        let Some(room_id) = self.player_rooms.get(&player_id) else {
            return false;
        };
        let Some(match_state) = self.matches.get(room_id) else {
            return false;
        };

        match_state.has_bot()
            && !match_state.is_done()
            && match_state.question_number() <= question_count()
    }
}
