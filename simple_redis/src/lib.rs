mod connection;
pub use connection::Connection;

mod delay;
pub use delay::delay;
pub use delay::Delay;
pub use delay::MiniTokio;
pub use delay::Task;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
