use rkyv::{Archive, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub enum Request {
    Handshake { uuid: Uuid },
    Disconnect { uuid: Uuid },
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub enum Response {
    HandshakeResponse,
}
