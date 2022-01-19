use crate::frame::{Frame};
use tokio::net::{TcpStream};
use std::io;
use bytes::Buf;
use tokio::io::{BufWriter, AsyncWriteExt};

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
}

impl Connection {
    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            
        }
    }

    pub fn new(socket: TcpStream ) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Array(val) => {

            }
            _ => self.write_value(frame).await?,
        }

        self.stream.flush().await
    }

    async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                // self.stream.write_u8(b'+').await?;
            }
            Frame::Error(val) => {
            }
            Frame::Integer(val) => {

            }
            Frame::Bulk(val) => {

            }
            Frame::Null => {

            }
            Frame::Array(val) => unreachable!(),
        }
        Ok(())
    }

}
