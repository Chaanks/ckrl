use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use glow::{Context as GlowContext, HasContext};

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

    fn run(self) {
        let event_loop = self.ctx.window.el;
        let windowed_context = self.ctx.window.wc;
        let gl = self.ctx.gl;

        println!(
            "Pixel format of the window's GL context: {:?}",
            windowed_context.get_pixel_format()
        );

        let vertices = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ];

        unsafe {
            let vbo: u32;
            let id = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(id));
            //gl.buffer_data();
        }

        event_loop.run(move |event, _, control_flow| {
            //println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            unsafe {
                gl.clear_color(0.2, 0.3, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        windowed_context.resize(physical_size)
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}


fn main() {

    ckrl::start_logger();

    match MyApp::new() {
        Ok(app) => app.run(),
        Err(err) => error!("Failed to create application. Cause: {}", err),
    }

}