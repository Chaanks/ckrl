pub mod window;
pub mod context;
pub mod logger;
pub mod gl;

pub use self::logger::start_logger;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;