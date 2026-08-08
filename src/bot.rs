use crate::state::SharedState;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};
use uuid::Uuid;

pub fn maybe_start(player_id: Uuid, state: SharedState) {
    tokio::spawn(async move {
        sleep(Duration::from_secs(3 + random(7) as u64)).await;
        if state.lock().await.match_with_bot_if_waiting(player_id) {
            answer_loop(player_id, state);
        }
    });
}

fn answer_loop(player_id: Uuid, state: SharedState) {
    tokio::spawn(async move {
        while state.lock().await.should_bot_answer(player_id) {
            sleep(Duration::from_millis(800 + random(1700) as u64)).await;
            state.lock().await.answer_bot(player_id, random);
        }
    });
}

fn random(max: usize) -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize
        % max
}
