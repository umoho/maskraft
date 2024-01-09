use crate::{packet::BytesParser, Result};

use super::Serverbound;

#[derive(Debug)]
pub struct StatusRequest;

impl Serverbound for StatusRequest {
    fn decoder(_: &mut BytesParser) -> Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct StatusPingRequest {
    pub payload: i64,
}

impl Serverbound for StatusPingRequest {
    fn decoder(parser: &mut BytesParser) -> Result<Self> {
        let payload = parser.next()?;
        Ok(Self { payload })
    }
}
