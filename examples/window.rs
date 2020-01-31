use log::{info, error};

use ckrl::context::{Context, ContextBuilder};
use ckrl::window::InitHints;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const WINDOW_TILE: &'static str = "Hello window";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;


struct MyApp {
    ctx: Context
}

impl MyApp {
    fn new() -> Result<Self> {
        info!("Creation application");

        let ctx = ContextBuilder::new()
            .with_title(WINDOW_TILE)
            .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .with_hints(InitHints::default())
            .build()?;

        Ok(Self {
            ctx
        })
    }
}


fn main() {

    ckrl::start_logger();

    match MyApp::new() {
        Ok(_) => info!("running"),
        Err(err) => error!("Failed to create application. Cause: {}", err),
    }

}