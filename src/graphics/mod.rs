use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Context {
    gl: glow::Context,
    window: Window,
}

struct Window {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

struct App {
    ctx: Context,
}

impl App {
    fn new() -> Result<Self> {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("A fantastic window!");

        let windowed_context = ContextBuilder::new()
            .build_windowed(wb, &el)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let gl = glow::Context::from_loader_function(|ptr| {
            windowed_context.get_proc_address(ptr) as *const _
        });

        Ok(App {
            ctx: Context {
                gl,
                window: Window {
                    el,
                    wc: windowed_context,
                },
            }
        })
    }
}