use crate::{Frame, Parse};

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Publish(Publish),
    Set(Set),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
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
}



#[derive(Debug)]
pub struct Publish {

}

#[derive(Debug)]
pub struct Set {

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
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;        
        let command_name = parse.next_string()?.to_lowercase();

        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            _ => {
                return Ok(Command::Unknown(Unknown::new(command_name)));
            }
        };

        Ok(command)
    }
}