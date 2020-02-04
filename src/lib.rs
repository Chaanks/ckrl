pub mod window;
pub mod context;
pub mod logger;
pub mod gl;
pub mod shader;
pub mod shader_string;

pub use self::logger::start_logger;
pub use self::shader_string::{FRAGMENT_SHADER, VERTEX_SHADER};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;