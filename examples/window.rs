
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::WindowedContext;
use log::{info};


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const WINDOW_TILE: &'static str = "Hello window";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;


struct MyApp;


impl MyApp {
    fn new() -> Result<Self> {
        info!("Creation application");
        Ok(Self)
    }

    fn run(&mut self, event_loop: EventLoop<()>) {
        let wb = WindowBuilder::new().with_title("A fantastic window!");

        let windowed_context = ContextBuilder::new().build_windowed(wb, &event_loop).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        println!(
            "Pixel format of the window's GL context: {:?}",
            windowed_context.get_pixel_format()
        );

        //let gl = support::load(&windowed_context.context());

        event_loop.run(move |event, _, control_flow| {
            //println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

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
                    //gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }

}


fn main() {

    ckrl::start_logger();
    info!("test");

    let el = EventLoop::new();

}