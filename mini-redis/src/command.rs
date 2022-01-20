use crate::{Frame, Parse, Db, Connection, Shutdown, ParseError};
use tracing::{debug, instrument};
// use crate::command::{Parse, ParseError};
use bytes::Bytes;
use std::time::Duration;

#[derive(Debug)]
pub enum Command {
    Get(Get),
    // Publish(Publish),
    Set(Set),
    // Subscribe(Subscribe),
    // Unsubscribe(Unsubscribe),
    Unknown(Unknown),
}

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Get> {
        let key = parse.next_string()?;
        Ok(Get { key })
    }

    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let response = if let Some(value) = db.get(&self.key) {
            Frame::Bulk(value)
        } else {
            Frame::Null
        };

        debug!(?response);

        dst.write_frame(&response).await?;
        Ok(())
    }
}



#[derive(Debug)]
pub struct Publish {

}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
    expire: Option<Duration>,
}

impl Set {
    pub fn key(&self) -> &str {
        &self.key
    }
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Set> {
        use ParseError::EndOfStream;

        let key = parse.next_string()?;
        let value = parse.next_bytes()?;

        let mut expire = None;

        match parse.next_string() {
            Ok(s) if s.to_uppercase() == "EX" => {
                let secs = parse.next_int()?;
                expire = Some(Duration::from_secs(secs));
            }
            Ok(s) if s.to_uppercase() == "PX" => {
                let ms = parse.next_int()?;
                expire = Some(Duration::from_millis(ms));
            }
            Ok(_) => return Err("currently `SET` only supports the expiration option".into()),
            Err(EndOfStream) => {}
            Err(err) => return Err(err.into()),
        }
        Ok(Set { key, value, expire }) 
    }

    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        db.set(self.key, self.value, self.expire);
        let response = Frame::Simple("OK".to_string());
        debug!(?response);
        dst.write_frame(&response).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Subscribe {

}

#[derive(Debug)]
pub struct Unsubscribe {

}

#[derive(Debug)]
pub struct Unknown {
    command_name: String,
}

impl Unknown {
    pub(crate) fn new(key: impl ToString) -> Unknown {
        Unknown {
            command_name: key.to_string(),
        }
    }

    #[instrument(skip(self, dst))]
    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        Ok(())
    }
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;        
        let command_name = parse.next_string()?.to_lowercase();

        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            _ => {
                return Ok(Command::Unknown(Unknown::new(command_name)));
            }
        };

        Ok(command)
    }

    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection, shutdown: &mut Shutdown) -> crate::Result<()> {
        use Command::*;

        match self {
            Get(cmd) => cmd.apply(db, dst).await,
            Unknown(cmd) => cmd.apply(dst).await,
            Set(cmd) => cmd.apply(db, dst).await,
        }
    }

}