use std::mem;
use std::rc::Rc;
use glow::{Context as GlowContext, HasContext};

use log::{info, error, debug};

use crate::Result;

type BufferId = <GlowContext as HasContext>::Buffer;
type ProgramId = <GlowContext as HasContext>::Program;
type VertexArrayId = <GlowContext as HasContext>::VertexArray;


pub struct GraphicsDevice {
    pub gl: Rc<GlowContext>,
    current_vertex_buffer: Option<BufferId>,
    current_index_buffer: Option<BufferId>,
    current_program: Option<ProgramId>,
    current_vertex_array: Option<VertexArrayId>,
}


impl GraphicsDevice {
    pub fn new(gl: GlowContext) -> Result<GraphicsDevice> {
        unsafe {
            let current_vertex_array = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(current_vertex_array));

            Ok(GraphicsDevice {
                gl: Rc::new(gl),

                current_vertex_buffer: None,
                current_index_buffer: None,
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
        index_buffer: &RawIndexBuffer,
        program: &RawProgram,
        count: i32,
        ) {
        unsafe {
            self.gl.bind_vertex_array(self.current_vertex_array);
            self.bind_vertex_buffer(Some(&vertex_buffer));
            self.bind_index_buffer(Some(&index_buffer));
            self.bind_program(Some(program));
            self.gl.draw_arrays(
                glow::TRIANGLES,
                0,
                count,
            );
            // self.gl.draw_elements(
            //     glow::TRIANGLES, 
            //     count, 
            //     glow::UNSIGNED_INT,
            //     0
            // );
        }
    }

    
    pub fn new_vertex_buffer(
        &mut self,
        count: usize,
        stride: usize,
        usage: BufferUsage,
    ) -> Result<RawVertexBuffer> {
        unsafe {
            info!("New vertex buffer with capacity: {} bytes", count);
            let id = self.gl.create_buffer()?;

            let buffer = RawVertexBuffer {
                gl: Rc::clone(&self.gl),
                id,
                count,
                stride,
            };

            self.bind_vertex_buffer(Some(&buffer));

            self.gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                count as i32,
                usage.into(),
            );

            debug!("Vertex buffer created with glGetError {}", self.gl.get_error());
            Ok(buffer)
        }

    }

    pub fn set_vertex_buffer_data(
        &mut self,
        buffer: &RawVertexBuffer,
        data: &[f32],
        offset: usize,
    ) {
        unsafe {    
            info!("Set vertex buffer data");
            self.bind_vertex_buffer(Some(&buffer));

            let u8_buffer = bytemuck::cast_slice(data);
        
            self.gl.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                (offset * mem::size_of::<f32>()) as i32,
                u8_buffer
            );

            debug!("Vertex data copied in buffer with glGetError {}", self.gl.get_error());
        }
    }

    pub fn set_vertex_buffer_attribute(
        &mut self,
        buffer: &RawVertexBuffer,
        index: u32,
        size: i32,
        offset: usize,
    ) {
        unsafe {
            info!("Set vertex buffer attribute");
            self.bind_vertex_buffer(Some(buffer));

            self.gl.vertex_attrib_pointer_f32(
                index,
                size,
                glow::FLOAT,
                false,
                (buffer.stride * mem::size_of::<f32>()) as i32,
                (offset * mem::size_of::<f32>()) as i32,
            );

            self.gl.enable_vertex_attrib_array(index);
            debug!("Vertex attribute enabled with glGetError {}", self.gl.get_error());
        }
    }


    pub fn new_index_buffer(
        &mut self,
        count: usize,
        usage: BufferUsage,
    ) -> Result<RawIndexBuffer> {
         unsafe {
            info!("New index buffer with capacity: {} bytes", count);
             let id = self.gl.create_buffer()?;

            let buffer = RawIndexBuffer {
                gl: Rc::clone(&self.gl),
                id,
                count,
            };

            self.bind_index_buffer(Some(&buffer));

            self.gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                (count * mem::size_of::<u32>()) as i32,
                usage.into(),
            );

            debug!("Index buffer created with glGetError {}", self.gl.get_error());
            Ok(buffer)
        }
    }

    pub fn set_index_buffer_data(
        &mut self,
        buffer: &RawIndexBuffer,
        data: &[u32],
        offset: usize,
    ) {
        unsafe {
            info!("Set index buffer data");
            self.bind_index_buffer(Some(&buffer));

            let u8_buffer = bytemuck::cast_slice(data);
            println!("u8_buffer: {:?}", u8_buffer);

            let byte_len = std::mem::size_of_val(data) / std::mem::size_of::<u8>();
            let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);
            println!("byte_slice: {:?}", byte_slice);

            self.gl.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                (offset * mem::size_of::<f32>()) as i32,
                u8_buffer
            );
            debug!("Index data copied in buffer with glGetError {}", self.gl.get_error());
        }
    }

    pub fn new_program(
        &mut self,
        vertex_code: &str,
        fragment_code: &str
    ) -> Result<RawProgram> {
        // compile shaders from strings
        unsafe {
            info!("New shader program");
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
            
            debug!("Shader program created with glGetError {}", self.gl.get_error());
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

    pub fn bind_index_buffer(&mut self, buffer: Option<&RawIndexBuffer>) {
        unsafe {
            let id = buffer.map(|x| x.id);
    
            if self.current_index_buffer != id {
                self.gl.bind_buffer(glow::ARRAY_BUFFER, id);
                self.current_index_buffer = id;
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

macro_rules! handle_impls {
    ($name:ty, $delete:ident) => {
        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                self.id == other.id
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    self.gl.$delete(self.id);
                }
            }
        }
    };
}

#[derive(Clone, Copy)]
pub enum BufferUsage {
    StaticDraw,
    DynamicDraw,
}

impl From<BufferUsage> for u32 {
    fn from(buffer_usage: BufferUsage) -> u32{
        match buffer_usage {
            BufferUsage::StaticDraw => glow::STATIC_DRAW,
            BufferUsage::DynamicDraw => glow::DYNAMIC_DRAW,
        }
    }
}

#[derive(Debug)]
pub struct RawVertexBuffer {
    gl: Rc<GlowContext>,
    id: BufferId,
    count: usize,
    stride: usize,
}

handle_impls!(RawVertexBuffer, delete_buffer);

#[derive(Debug)]
pub struct RawIndexBuffer {
    gl: Rc<GlowContext>,
    id: BufferId,
    count: usize,
}

handle_impls!(RawIndexBuffer, delete_buffer);

pub struct RawProgram {
    id: ProgramId
}