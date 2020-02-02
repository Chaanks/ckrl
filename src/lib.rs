pub mod window;
pub mod context;
pub mod logger;
pub mod gl;
pub mod shader;
pub mod shader_string;

pub use self::logger::start_logger;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;