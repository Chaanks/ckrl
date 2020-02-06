use glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder, PixelFormat};
use glow::Context as GlowContext;
use crate::Result;
use crate::context;

pub struct Window {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

impl Window {
    pub fn new(settings: &context::ContextBuilder) -> Result<(Window, GlowContext)> {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title(settings.window_title.clone())
            .with_inner_size(LogicalSize::new(
                settings.window_width,
                settings.window_height
            ));
    
        let windowed_context = ContextBuilder::new()
            .with_gl(settings.platform_hints.gl_version)
            .with_gl_profile(settings.platform_hints.gl_profile)
            .with_hardware_acceleration(Some(settings.platform_hints.hardware_acceleration))
            .with_vsync(settings.platform_hints.vsync)
            .with_srgb(settings.platform_hints.srgb)
            .build_windowed(wb, &el)
            .unwrap();
    
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    
        if settings.platform_hints.fullscreen {
            let mh = el.available_monitors().nth(0).unwrap();
            windowed_context
            .window()
            .set_fullscreen(Some(glutin::window::Fullscreen::Borderless(mh)));
        }
    
        let gl = GlowContext::from_loader_function(|ptr| {
            windowed_context.get_proc_address(ptr) as *const _
        });

        Ok((Self {
            el,
            wc: windowed_context,
        }, gl))
    }

    pub fn get_pixel_format(&self) -> PixelFormat {
        return self.wc.get_pixel_format();
    }
}

#[derive(Debug)]
pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
    pub gl_version: glutin::GlRequest,
    pub gl_profile: glutin::GlProfile,
    pub hardware_acceleration: bool,
    pub srgb: bool,
}

impl Default for InitHints {
    fn default() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            gl_version: glutin::GlRequest::Latest,
            gl_profile: glutin::GlProfile::Core,
            hardware_acceleration: true,
            srgb: true,
        }
    }
}