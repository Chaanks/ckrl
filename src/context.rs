use crate::window::{Window, InitHints};
use crate::Result;


pub struct Context {
    pub gl: glow::Context,
    pub window: Window,
}

impl Context {
    fn new(settings: &ContextBuilder) -> Result<Context> {
        let (window, gl_context) = Window::new(settings)?;
        
        Ok(Self {
            gl: gl_context,
            window,
        })
        
    }
}


#[derive(Debug)]
pub struct ContextBuilder {
    pub(crate) window_title: String,
    pub(crate) window_width: u32,
    pub(crate) window_height: u32,
    pub(crate) platform_hints: InitHints,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title<S>(&mut self, title: S) -> &mut Self
    where
        S: ToString,
    {
        self.window_title = title.to_string();
        self
    }

    pub fn with_size(
        &mut self,
        window_width: u32,
        window_height: u32
    ) -> &mut Self {
        self.window_width = window_width;
        self.window_height = window_height;
        self
    }

    pub fn with_hints(&mut self, platform_hints: InitHints) -> &mut Self {
        self.platform_hints = platform_hints;
        self
    }

    pub fn build(&self) -> Result<Context> {
        Context::new(self)
    }
}


impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            window_title: "ckrl".into(),
            window_width: 800,
            window_height: 600,
            platform_hints: InitHints::default(),
        }
    }
}