use crate::datatype::VarInt;

pub mod login;
pub mod status;

pub trait Clientbound: Sized {
    fn packet_id() -> VarInt;
}
