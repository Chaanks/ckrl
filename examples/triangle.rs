use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use glow::{Context as GlowContext, HasContext};

use log::{info, error};

use ckrl::context::{Context, ContextBuilder};
use ckrl::window::InitHints;
use ckrl::gl::BufferUsage;


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
        println!(
            "Pixel format of the window's GL context: {:?}",
           self.ctx.window.get_pixel_format()
        );
        
        let event_loop = self.ctx.window.el;
        let windowed_context = self.ctx.window.wc;
        let mut device = self.ctx.device;

        let vertices: [f32; 12] = [
            -0.5,  0.5, 0.0,  // top right
            0.5, 0.5, 0.0,  // bottom right
           0.5, -0.5, 0.0,  // bottom let
           -0.5,  -0.5, 0.0,   // top left 
        ];

        let indices: [u32; 6] = [
            0, 1, 2, // first triangle
            2, 3, 0, // second triangle
        ];


        let buffer = device.new_vertex_buffer(48, 3, BufferUsage::StaticDraw).expect("Failed to create vertex buffer");
        device.set_vertex_buffer_data(&buffer, &vertices, 0);
        device.set_vertex_buffer_attribute(&buffer, 0, 3, 0);

        let index = device.new_index_buffer(24, BufferUsage::StaticDraw).expect("Failed to create index buffer");
        device.set_index_buffer_data(&index, &indices, 0);

        let program = device.new_program(ckrl::VERTEX_SHADER, ckrl::FRAGMENT_SHADER).expect("Failed to create shader program");

        event_loop.run(move |event, _, control_flow| {
            //println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            device.clear(0.2, 0.3, 0.3, 1.0);
            device.draw(&buffer, &index, &program);

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