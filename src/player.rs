use crate::messages::ServerMessage;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Player {
    pub id: Uuid,
    pub tx: mpsc::UnboundedSender<ServerMessage>,
    pub is_bot: bool,
}

impl Player {
    pub fn send(&self, message: ServerMessage) {
        if self.is_bot {
            return;
        }

        let _ = self.tx.send(message);
    }
}
