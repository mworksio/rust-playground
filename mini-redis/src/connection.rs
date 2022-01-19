use crate::frame::{Frame};
use tokio::net::{TcpStream};

#[derive(Debug)]
pub struct Connection {

}

impl Connection {
    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            
        }
    }

    pub fn new(socket: TcpStream ) -> Connection {
        Connection {

        }
    }

}
