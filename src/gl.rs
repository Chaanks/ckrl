use std::mem;
use std::rc::Rc;
use glow::{Context as GlowContext, HasContext};

use log::{info, error};

use crate::Result;

type BufferId = <GlowContext as HasContext>::Buffer;
type ProgramId = <GlowContext as HasContext>::Program;
type VertexArrayId = <GlowContext as HasContext>::VertexArray;

#[derive(Debug)]
pub struct CtxWrapper(*const GlowContext);
impl CtxWrapper {
    pub fn new(ctx: &GlowContext) -> Self {
        Self(ctx as *const GlowContext)
    }
}

pub struct GraphicsDevice {
    pub gl: Rc<GlowContext>,
    current_vertex_buffer: Option<BufferId>,
    current_program: Option<ProgramId>,
    current_vertex_array: Option<VertexArrayId>,
}


impl GraphicsDevice {
    pub fn new(gl: GlowContext) -> Result<GraphicsDevice> {
        unsafe {
            // This is only needed for Core GL - if we wanted to be uber compatible, we'd
            // turn it off on older versions.
            let current_vertex_array = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(current_vertex_array));

            // TODO: Find a nice way of exposing this via the platform layer
            // println!("Swap Interval: {:?}", video.gl_get_swap_interval());

            Ok(GraphicsDevice {
                gl: Rc::new(gl),

                current_vertex_buffer: None,
                current_program: None,
                current_vertex_array: Some(current_vertex_array),
            })
        }
    }

    pub fn get_renderer(&self) -> String {
        unsafe { self.gl.get_parameter_string(glow::RENDERER) }
    }

    pub fn get_version(&self) -> String {
        unsafe { self.gl.get_parameter_string(glow::VERSION) }
    }

    pub fn get_vendor(&self) -> String {
        unsafe { self.gl.get_parameter_string(glow::VENDOR) }
    }

    pub fn get_shading_language_version(&self) -> String {
        unsafe { self.gl.get_parameter_string(glow::SHADING_LANGUAGE_VERSION) }
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.clear_color(r, g, b, a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(
        &mut self,
        vertex_buffer: &RawVertexBuffer,
        program: &RawProgram,
        ) {
        unsafe {
            self.gl.bind_vertex_array(self.current_vertex_array);
            self.bind_vertex_buffer(Some(&vertex_buffer));
            self.bind_program(Some(program));
            self.gl.draw_arrays(
                glow::TRIANGLES,
                0,
                3,
            );
        }
    }

    pub fn new_vertex_buffer_(
        &mut self,
        data: Option<&[f32]>,
    ) -> Result<RawVertexBuffer> {
        unsafe {
            let id = self.gl.create_buffer()?;
    
            let buffer = RawVertexBuffer {
                ctx: CtxWrapper::new(&self.gl),
                id,
                size: 36,
                stride: 3,
            };
    
            self.bind_vertex_buffer(Some(&buffer));
    
            let u8_buffer = bytemuck::cast_slice(data.unwrap());
            let size = u8_buffer.len();
    
            self.gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                size as i32,
                glow::STREAM_DRAW,
            );
    
            self.gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, u8_buffer);
    
            Ok(buffer)
        }
    }

    pub fn set_vertex_buffer_attribute(
        &mut self,
        buffer: &RawVertexBuffer,
    ) {
        unsafe {
            self.bind_vertex_buffer(Some(buffer));

            info!("glGetError0 {}", self.gl.get_error());

            self.gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                (buffer.stride * mem::size_of::<f32>()) as i32,
                0,
            );
            info!("glGetError {}", self.gl.get_error());
            self.gl.enable_vertex_attrib_array(0);
        }
    }

    pub fn new_program(
        &mut self,
        vertex_code: &str,
        fragment_code: &str
    ) -> Result<RawProgram> {
        // compile shaders from strings
        unsafe {
            // vertex shader
            let vertex_id = self.gl.create_shader(glow::VERTEX_SHADER)?;
            self.gl.shader_source(vertex_id, &vertex_code);
            self.gl.compile_shader(vertex_id);
            if !self.gl.get_shader_compile_status(vertex_id) {
                error!("Failed to compile vertex shader");
                return Err(failure::err_msg(self.gl.get_shader_info_log(vertex_id)).into());
            }
            // fragment shader
            let fragment_id = self.gl.create_shader(glow::FRAGMENT_SHADER)?;
            self.gl.shader_source(fragment_id, &fragment_code);
            self.gl.compile_shader(fragment_id);
            if !self.gl.get_shader_compile_status(fragment_id) {
                error!("Failed to compile fragment shader");
                return Err(failure::err_msg(self.gl.get_shader_info_log(fragment_id)).into());
            }
    
            // shader program
            let program_id = self.gl.create_program()?;
            self.gl.attach_shader(program_id, vertex_id);
            self.gl.attach_shader(program_id, fragment_id);
            self.gl.link_program(program_id);
            if !self.gl.get_program_link_status(program_id) {
                error!("Failed to link program");
                return Err(failure::err_msg(self.gl.get_program_info_log(program_id)).into());
            }
    
            self.gl.delete_shader(vertex_id);
            self.gl.delete_shader(fragment_id);
            
            Ok(RawProgram { id:program_id })
        }
    }

    pub fn bind_vertex_buffer(&mut self, buffer: Option<&RawVertexBuffer>) {
        unsafe {
            let id = buffer.map(|x| x.id);
    
            if self.current_vertex_buffer != id {
                self.gl.bind_buffer(glow::ARRAY_BUFFER, id);
                self.current_vertex_buffer = id;
            }
        }
    }

    fn bind_program(&mut self, program: Option<&RawProgram>) {
        unsafe {
            let id = program.map(|x| x.id);

            if self.current_program != id {
                self.gl.use_program(id);
                self.current_program = id;
            }
        }
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

pub struct RawProgram {
    id: ProgramId
}