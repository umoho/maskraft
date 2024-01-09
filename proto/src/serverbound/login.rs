use crate::{packet::BytesParser, Result};

use super::Serverbound;

#[derive(Debug)]
pub struct LoginStart {
    pub name: String,
    // pub player_uuid: (), // todo: uuid decoding
}

impl Serverbound for LoginStart {
    fn decoder(parser: &mut BytesParser) -> Result<Self> {
        let name = parser.next()?;
        Ok(Self { name })
    }
}
