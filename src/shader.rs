use glow::{Context as GlowContext, HasContext};
use log::error;

use crate::Result;

type ProgramId = <GlowContext as HasContext>::Program;

pub struct RawProgram {
    id: ProgramId
}

impl RawProgram {
    pub fn bind(&self, gl: &GlowContext) {
        unsafe {
            gl.use_program(Some(self.id));
        }
    }
}


pub fn new_program(
    gl: &GlowContext,
    vertex_code: &str,
    fragment_code: &str
) -> Result<RawProgram> {
    // compile shaders from strings
    unsafe {
        // vertex shader
        let vertex_id = gl.create_shader(glow::VERTEX_SHADER)?;
        gl.shader_source(vertex_id, &vertex_code);
        gl.compile_shader(vertex_id);
        if !gl.get_shader_compile_status(vertex_id) {
            error!("Failed to compile vertex shader");
            return Err(failure::err_msg(gl.get_shader_info_log(vertex_id)).into());
        }

        // fragment shader
        let fragment_id = gl.create_shader(glow::FRAGMENT_SHADER)?;
        gl.shader_source(fragment_id, &fragment_code);
        gl.compile_shader(fragment_id);
        if !gl.get_shader_compile_status(fragment_id) {
            error!("Failed to compile fragment shader");
            return Err(failure::err_msg(gl.get_shader_info_log(fragment_id)).into());
        }

        // shader program
        let program_id = gl.create_program()?;
        gl.attach_shader(program_id, vertex_id);
        gl.attach_shader(program_id, fragment_id);
        gl.link_program(program_id);
        if !gl.get_program_link_status(program_id) {
            error!("Failed to link program");
            return Err(failure::err_msg(gl.get_program_info_log(program_id)).into());
        }

        gl.delete_shader(vertex_id);
        gl.delete_shader(fragment_id);
        
        Ok(RawProgram { id:program_id })
    }
}

    
