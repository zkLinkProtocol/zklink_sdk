use serde::{Deserialize, Serialize};

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
