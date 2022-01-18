pub const DEFAULT_PORT: &str = "6379";
pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub mod server;