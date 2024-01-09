use crate::datatype::{Encode, Length};

#[derive(Debug)]
pub struct Disconnect {
    pub reason: serde_json::Value,
}

impl Encode for Disconnect {
    fn encode(&self) -> Vec<u8> {
        let json_str = serde_json::to_string(&self.reason).expect("Parsing JSON failed");
        json_str.encode()
    }
}

impl Length<i32> for Disconnect {
    fn length(&self) -> i32 {
        self.encode().len() as i32
    }
}
