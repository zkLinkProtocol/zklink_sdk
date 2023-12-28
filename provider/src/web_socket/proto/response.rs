use serde::{Deserialize, Serialize};
use warp::ws::Message;

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub listen_key: String,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TopicsResponse<T> {
    pub topics: Vec<T>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct WsResponse<T> {
    pub result: T,
    pub error_code: usize,
    pub error_msg: String,
    pub id: usize,
}

impl<T> WsResponse<T>
where
    T: Serialize,
{
    pub fn is_error(&self) -> bool {
        self.error_code == 0
    }

    pub fn ws_message(&self) -> Result<Message, serde_json::Error> {
        let s = serde_json::to_string(self)?;
        let msg = Message::text(s);
        Ok(msg)
    }
}
