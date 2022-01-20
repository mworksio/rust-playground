pub const DEFAULT_PORT: &str = "6379";
pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub mod server;

mod connection;
pub use connection::Connection;

mod command;
pub use command::Command;

mod frame;
pub use frame::Frame;

mod parse;
use parse::{Parse, ParseError};

mod db;
use db::Db;
use db::DbDropGuard;

mod shutdown;
use shutdown::Shutdown;