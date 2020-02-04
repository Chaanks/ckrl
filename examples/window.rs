use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use glow::{Context as GlowContext, HasContext};

use log::{info, error};

use ckrl::context::{Context, ContextBuilder};
use ckrl::window::InitHints;
use ckrl::gl as device;
use ckrl::shader;

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

        let vertices: [f32;9] = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ];


        let buffer = device::new_vertex_buffer(&gl, Some(&vertices)).expect("Failed to create vertex buffer");
        let vao: u32;
        unsafe { vao = gl.create_vertex_array().unwrap(); }
        device::set_vertex_buffer_attribute(&gl, &buffer);
        
        let program = shader::new_program(&gl, ckrl::VERTEX_SHADER, ckrl::FRAGMENT_SHADER).expect("Failed to create shader program");

        event_loop.run(move |event, _, control_flow| {
            //println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            unsafe {
                gl.clear_color(0.2, 0.3, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);

                program.bind(&gl);
                gl.bind_vertex_array(Some(vao));
                device::bind_vertex_buffer(&gl, Some(&buffer));
                gl.draw_elements(
                    glow::TRIANGLES,
                    3,
                    glow::UNSIGNED_INT,
                    0,
                );
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