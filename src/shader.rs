use glow::{Context as GlowContext, HasContext};
use log::error;

use crate::Result;

type ProgramId = <GlowContext as HasContext>::Program;

pub struct RawProgram {
    id: ProgramId
}


pub fn new_program(
    gl: &GlowContext,
    vertex_code: &str,
    fragment_code: &str
) -> Result<RawProgram> {
    // compile shaders from strings
    unsafe {
        // vertex shader
        let vertex = gl.create_shader(glow::VERTEX_SHADER)?;
        gl.shader_source(vertex, &vertex_code);
        gl.compile_shader(vertex);
        if !gl.get_shader_compile_status(vertex) {
            error!("Failed to compile vertex shader");
            return Err(failure::err_msg(gl.get_shader_info_log(vertex)).into());
        }

        // fragment shader
        
    }

    Ok(RawProgram { id:32 })

}
    
