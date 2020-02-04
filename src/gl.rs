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
        bind_vertex_buffer(gl, Some(buffer));
        info!("glGetError0 {}",gl.get_error());
        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            (buffer.stride * mem::size_of::<f32>()) as i32,
            0,
        );
        info!("glGetError {}",gl.get_error());
        gl.enable_vertex_attrib_array(0);
    }
}

pub fn new_vertex_buffer_(
    gl: &GlowContext,
    data: Option<&[f32]>,
) -> Result<RawVertexBuffer> {
    unsafe {
        let id = gl.create_buffer()?;
        info!("glGetError {}",gl.get_error());

        let buffer = RawVertexBuffer {
            ctx: CtxWrapper::new(gl),
            id,
            size: 36,
            stride: 3,
        };

        bind_vertex_buffer(gl, Some(&buffer));

        let u8_buffer = bytemuck::cast_slice(data.unwrap());
        let size = u8_buffer.len();

        gl.buffer_data_size(
            glow::ARRAY_BUFFER,
            size as i32,
            glow::STREAM_DRAW,
        );
        info!("glGetError {}",gl.get_error());


        gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, u8_buffer);
        info!("glGetError {}",gl.get_error());

        Ok(buffer)
    }
    //gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STREAM_DRAW);
    //let size = std::mem::size_of_val(data) / std::mem::size_of::<u8>();
    //let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, size);
}

pub fn new_vertex_buffer(
    gl: &GlowContext,
    data: Option<&[f32]>,
) -> Result<RawVertexBuffer> {
    info!("New vertex buffer");
    let mut size = 0;
    Ok(RawVertexBuffer {
        ctx:CtxWrapper::new(gl),
        id: unsafe {
            let id = gl.create_buffer()?;
            
            info!("id: {}", id); 
            if let Some(data) = data { 
                let u8_buffer = bytemuck::cast_slice(data);
                size = u8_buffer.len();

                //size= std::mem::size_of_val(data) / std::mem::size_of::<u8>();
                //let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, size);
                //gl.enable_vertex_attrib_array(id);
                info!("size {}", size);
                gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STREAM_DRAW);
                info!("glGetError {}",gl.get_error());
                //gl.buffer_data_u8_slice(id, u8_buffer, glow::ARRAY_BUFFER);
                gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, u8_buffer);
                info!("glGetError {}",gl.get_error());
            }
            id
        },
        size: size,
        stride: 3,  
        })
    
    //gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STREAM_DRAW);
    //let size = std::mem::size_of_val(data) / std::mem::size_of::<u8>();
    //let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, size);
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