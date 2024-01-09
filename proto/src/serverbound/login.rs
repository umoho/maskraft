use crate::{datatype::Uuid, packet::BytesParser, Result};

use super::Serverbound;

#[derive(Debug)]
pub struct LoginStart {
    pub name: String,
    pub player_uuid: Uuid,
}

impl Serverbound for LoginStart {
    fn decoder(parser: &mut BytesParser) -> Result<Self> {
        let name = parser.next()?;
        let player_uuid = parser.next()?;
        Ok(Self { name, player_uuid })
    }
}
