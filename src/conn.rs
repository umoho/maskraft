use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    state: i32,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream, state: 0 }
    }

    pub fn get_stream<'t>(&'t self) -> &'t TcpStream {
        &self.stream
    }

    pub fn set_state(&mut self, state: i32) {
        self.state = state;
    }

    pub fn handle<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Self),
    {
        handler(&mut self);
        self
    }

    pub fn handle_when<F>(mut self, state: i32, mut handler: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        if state == self.state {
            handler(&mut self);
        }
        self
    }
}
