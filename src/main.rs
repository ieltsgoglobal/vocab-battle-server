mod bot;
mod match_state;
mod messages;
mod player;
mod questions;
mod state;
mod ws;

use axum::{Router, response::Html, routing::get};
use state::AppState;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use ws::ws_handler;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env.local").ok();

    let state = Arc::new(Mutex::new(AppState::new()));
    let app = Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3335".to_string());
    let addr = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html>
<body>
<h1 id="status">connecting</h1>
<script>
const status = document.getElementById("status");
const ws = new WebSocket(`ws://${location.host}/ws`);
ws.onmessage = (event) => status.textContent = event.data;
ws.onclose = () => status.textContent += " closed";
</script>
</body>
</html>"#,
    )
}
