use crate::{
    bot,
    messages::{ClientMessage, ServerMessage},
    player::Player,
    state::SharedState,
};
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use tokio::sync::mpsc;
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    let id = Uuid::new_v4();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let queued = state.lock().await.join(Player {
        id,
        tx,
        is_bot: false,
    });
    if queued {
        bot::maybe_start(id, state.clone());
    }

    loop {
        tokio::select! {
            Some(message) = rx.recv() => send_json(&mut socket, message).await,
            message = socket.recv() => match message {
                Some(Ok(Message::Text(text))) => handle_text(id, text.as_str(), &state).await,
                Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                Some(Ok(_)) => {}
            },
        }
    }

    state.lock().await.leave(id);
}

async fn handle_text(id: Uuid, text: &str, state: &SharedState) {
    let Ok(ClientMessage::Answer { answer }) = serde_json::from_str(text) else {
        return;
    };

    state.lock().await.answer(id, answer);
}

async fn send_json(socket: &mut WebSocket, message: ServerMessage) {
    let text = serde_json::to_string(&message).unwrap();
    let _ = socket.send(Message::Text(text.into())).await;
}
