pub use serde_json;

pub mod clientbound;
pub mod datatype;
pub mod packet;
pub mod serverbound;

pub const MAX_PACKET_SIZE: usize = 2097151;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DecodeError(&'static str),
}
