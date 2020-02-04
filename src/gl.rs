use std::mem;
use glow::{Context as GlowContext, HasContext};

use log::{info, debug};

use crate::Result;

type BufferId = <GlowContext as HasContext>::Buffer;


#[derive(Debug)]
pub struct CtxWrapper(*const GlowContext);
impl CtxWrapper {
    pub fn new(ctx: &GlowContext) -> Self {
        Self(ctx as *const GlowContext)
    }
}

pub fn set_vertex_buffer_attribute(
    gl: &GlowContext,
    buffer: &RawVertexBuffer,
) {
    unsafe {
        let index = 0;
        let size = buffer.size as i32;
        let data_type = glow::FLOAT;
        let normalized = false;
        let stride = (buffer.stride * mem::size_of::<f32>()) as i32;
        let offset = 0;

        gl.vertex_attrib_pointer_f32(
            index,
            size,
            data_type,
            normalized,
            stride,
            offset
        );

        gl.enable_vertex_attrib_array(index);

    }
}


pub fn new_vertex_buffer(
    gl: &GlowContext,
    data: Option<&[f32]>,
) -> Result<RawVertexBuffer> {
    info!("New vertex buffer");
    unsafe {
        let id = gl.create_buffer()?;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(id));

        if let Some(data) = data {
            //let u8_buffer = bytemuck::cast_slice(data);
            //size = u8_buffer.len();
            
            //gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STREAM_DRAW);
            let stride = 3;
            let size = std::mem::size_of_val(data) / std::mem::size_of::<u8>();
            info!("buffer size: {}", size);
            let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, size);
        
            let buffer = RawVertexBuffer {
                ctx: CtxWrapper::new(gl),
                id,
                size,
                stride,

            };

            bind_vertex_buffer(gl, Some(&buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, byte_slice, glow::STREAM_DRAW);

            return Ok(buffer);

        }

        Err(failure::err_msg("No data").into())
    }
}

pub fn bind_vertex_buffer(gl: &GlowContext, buffer: Option<&RawVertexBuffer>) {
    unsafe {
        let id = buffer.map(|x| x.id);
        gl.bind_buffer(glow::ARRAY_BUFFER, id);
    }
}


#[derive(Debug)]
pub struct RawVertexBuffer {
    ctx: CtxWrapper,
    id: BufferId,
    size: usize,
    stride: usize,
}

impl Drop for RawVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            (*self.ctx.0).delete_buffer(self.id);
        }
    }
}