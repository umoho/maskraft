use std::net::TcpStream;

use num_enum::{IntoPrimitive, TryFromPrimitive};

pub struct Connection {
    stream: TcpStream,
    state: State,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: State::Init,
        }
    }

    pub fn stream<'t>(&'t self) -> &'t TcpStream {
        &self.stream
    }

    pub fn switch_state(&mut self, state: State) {
        self.state = state;
        log::debug!("Connection state switched to {:?}", self.state);
    }

    pub fn handle<F>(mut self, mut handler: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        handler(&mut self);
        self
    }

    pub fn handle_when<F>(mut self, state: State, mut handler: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        if state == self.state {
            handler(&mut self);
        }
        self
    }
}

#[derive(Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum State {
    Init,
    Status = 0x01,
    Login = 0x02,
}
