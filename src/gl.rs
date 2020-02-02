use glow::{Context as GlowContext, HasContext};
use std::mem;
use std::rc::Rc;

use crate::Result;

type BufferId = <GlowContext as HasContext>::Buffer;


#[derive(Debug)]
pub struct CtxWrapper(*const GlowContext);
impl CtxWrapper {
    pub fn new(ctx: &GlowContext) -> Self {
        Self(ctx as *const GlowContext)
    }
}

pub fn new_vertex_buffer(
    gl: &GlowContext,
    data: Option<&[f32]>,
) -> Result<RawVertexBuffer> {
    unsafe {
        let id = gl.create_buffer()?;
        let mut size = 0;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(id));

        if let Some(data) = data {
            let u8_buffer = bytemuck::cast_slice(data);
            size = u8_buffer.len();
            gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STREAM_DRAW);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STREAM_DRAW);
        }
        
        Ok(RawVertexBuffer {
            ctx: CtxWrapper::new(gl),
            id,
            size,
        })
    }
}


#[derive(Debug)]
pub struct RawVertexBuffer {
    ctx: CtxWrapper,
    id: BufferId,
    size: usize,
}

impl Drop for RawVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            (*self.ctx.0).delete_buffer(self.id);
        }
    }
}