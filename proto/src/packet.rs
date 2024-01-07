use crate::{
    datatype::{Decode, Encode, Length, VarInt},
    serverbound::Serverbound,
    Result,
};

pub struct BytesParser<'b> {
    cur_start: usize,
    bytes: &'b [u8],
}

impl<'b> BytesParser<'b> {
    pub fn new(bytes: &'b [u8]) -> Self {
        Self {
            cur_start: 0,
            bytes,
        }
    }

    pub fn next<T>(&mut self) -> std::result::Result<T, <T as Decode>::Error>
    where
        T: Decode,
    {
        <T as Decode>::decode_streaming(&self.bytes[self.cur_start..]).map(|(idx, val)| {
            self.cur_start += idx;
            val
        })
    }
}

#[derive(Debug)]
pub struct Packet<T> {
    pub length: VarInt,
    pub packet_id: VarInt,
    pub data: T,
}

impl<T> Packet<T>
where
    T: Serverbound,
{
    pub fn decode<F>(bytes: &[u8], data_decoder: F) -> Result<Self>
    where
        F: FnOnce(&mut BytesParser) -> Result<T>,
    {
        PacketParser::new(bytes).parse(data_decoder)
    }
}

impl<T> Packet<T>
where
    T: Length<i32>,
{
    pub fn new(packet_id: impl Into<VarInt>, data: T) -> Self {
        let packet_id = packet_id.into();
        let length = VarInt(packet_id.length() + data.length());
        Self {
            length,
            packet_id,
            data,
        }
    }
}

impl<T> Encode for Packet<T>
where
    T: Encode,
{
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(&self.length.encode());
        buf.extend(&self.packet_id.encode());
        buf.extend(&self.data.encode());
        buf
    }
}

pub struct PacketParser<'p> {
    bytes_parser: BytesParser<'p>,
    packet_id: Option<VarInt>,
}

impl<'p> PacketParser<'p> {
    pub fn new(bytes: &'p [u8]) -> Self {
        Self {
            bytes_parser: BytesParser::new(bytes),
            packet_id: None,
        }
    }

    pub(crate) fn parse<T, F>(&mut self, parse_data: F) -> Result<Packet<T>>
    where
        F: FnOnce(&mut BytesParser<'p>) -> Result<T>,
    {
        let (length, packet_id) = self.parse_header()?;
        let data = parse_data(&mut self.bytes_parser)?;
        Ok(Packet {
            length,
            packet_id,
            data,
        })
    }

    pub(crate) fn parse_header(&mut self) -> Result<(VarInt, VarInt)> {
        let length = self.bytes_parser.next()?;
        let packet_id = self.bytes_parser.next()?;
        Ok((length, packet_id))
    }

    pub fn packet_id(&mut self) -> Result<VarInt> {
        let Some(packet_id) = self.packet_id else {
            let (_, packet_id) = self.parse_header()?;
            self.packet_id = Some(packet_id);
            return Ok(packet_id);
        };
        Ok(packet_id)
    }
}
