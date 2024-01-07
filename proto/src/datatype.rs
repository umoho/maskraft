pub trait Encode {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decode: Sized {
    type Error;

    fn decode(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::decode_streaming(bytes).map(|(_, val)| val)
    }

    /// Return a result with a tuple that
    /// the first element is the number of bytes this decoder ate, or error.
    fn decode_streaming(bytes: &[u8]) -> Result<(usize, Self), Self::Error>;
}

pub trait Length<T> {
    fn length(&self) -> T;
}

impl Decode for u16 {
    type Error = crate::Error;

    fn decode_streaming(bytes: &[u8]) -> Result<(usize, Self), Self::Error> {
        let unsigned_short = ((bytes[0] as u16) << 8) | (bytes[1] as u16);
        Ok((2, unsigned_short))
    }
}

impl Decode for i64 {
    type Error = crate::Error;

    fn decode_streaming(bytes: &[u8]) -> Result<(usize, Self), Self::Error> {
        let long = ((bytes[0] as i64) << 56)
            | ((bytes[1] as i64) << 48)
            | ((bytes[2] as i64) << 40)
            | ((bytes[3] as i64) << 32)
            | ((bytes[4] as i64) << 24)
            | ((bytes[5] as i64) << 16)
            | ((bytes[6] as i64) << 8)
            | (bytes[7] as i64);
        Ok((8, long))
    }
}

const SEGMENT_BITS: u8 = 0x7f;
const CONTINUE_BIT: u8 = 0x80;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VarInt(pub i32);

impl Encode for VarInt {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut result = self.0;
        loop {
            if (result & !SEGMENT_BITS as i32) == 0 {
                bytes.push(result as u8);
                return bytes;
            }
            bytes.push(((result & SEGMENT_BITS as i32) | CONTINUE_BIT as i32) as u8);
            result >>= 7;
        }
    }
}

impl Length<i32> for VarInt {
    fn length(&self) -> i32 {
        self.encode().len() as i32
    }
}

impl Decode for VarInt {
    type Error = crate::Error;

    fn decode_streaming(bytes: &[u8]) -> Result<(usize, Self), Self::Error> {
        let mut idx = 0;
        let mut pos = 0;
        let mut result = 0;
        let mut current_byte;

        for byte in bytes {
            idx += 1;
            current_byte = *byte as i32;
            result |= (current_byte & SEGMENT_BITS as i32) << pos;

            if (current_byte & CONTINUE_BIT as i32) == 0 {
                break;
            }

            pos += 7;
            if pos >= 32 {
                return Err(crate::Error::DecodeError("VarInt is too big")); // todo: need more testing
            }
        }

        Ok((idx, Self(result)))
    }
}

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<i32> for VarInt {
    fn eq(&self, other: &i32) -> bool {
        &self.0 == other
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[test]
fn test_var_int() {
    let integers = vec![
        0,
        1,
        2,
        127,
        128,
        255,
        25565,
        2097151,
        2147483647,
        // -1, // todo: something wrong here
        -2147483648,
    ];
    for i in integers {
        let var_int = VarInt(i);
        let bytes = var_int.encode();
        println!("{:0x?}", &bytes);
        println!("{:?}", VarInt::decode(&bytes));
    }
}

impl Encode for String {
    fn encode(&self) -> Vec<u8> {
        let len = self.len();
        let mut bytes = Vec::new();
        bytes.extend(VarInt(len as i32).encode());
        bytes.extend(self.as_bytes());
        bytes
    }
}

impl Decode for String {
    type Error = crate::Error;

    fn decode_streaming(bytes: &[u8]) -> Result<(usize, Self), Self::Error> {
        let (str_start, str_len) = VarInt::decode_streaming(&bytes)?;
        let str_end = str_start + str_len.0 as usize;
        let string = std::str::from_utf8(&bytes[str_start..str_end])
            .map_err(|_| crate::Error::DecodeError("Utf8Error while decoding string"))?;
        Ok((str_end, String::from(string)))
    }
}

#[test]
fn test_string() {
    let string = "hello world";
    let bytes = string.to_string().encode();
    println!("{:0x?}", &bytes);
    println!("{:?}", String::decode_streaming(&bytes));
}