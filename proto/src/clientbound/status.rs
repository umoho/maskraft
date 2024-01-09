use crate::datatype::{Encode, Length, VarInt};

use super::Clientbound;

#[derive(Debug)]
pub struct StatusResponse {
    pub json_response: serde_json::Value,
}

impl Encode for StatusResponse {
    fn encode(&self) -> Vec<u8> {
        let json_str = serde_json::to_string(&self.json_response).expect("Parsing JSON failed");
        json_str.encode()
    }
}

impl Length<i32> for StatusResponse {
    fn length(&self) -> i32 {
        self.encode().len() as i32
    }
}

impl Clientbound for StatusResponse {
    fn packet_id() -> VarInt {
        VarInt(0x00)
    }
}
