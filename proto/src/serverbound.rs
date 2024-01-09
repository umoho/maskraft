use crate::{packet::BytesParser, Result};

pub mod handshake;
pub mod login;
pub mod status;

pub trait Serverbound: Sized {
    fn decoder(parser: &mut BytesParser) -> Result<Self>;
}
