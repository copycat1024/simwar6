use crate::gl::{
    enums::{ShaderParameterName, ShaderType},
    Gl,
};
use std::ptr::null;

pub struct Shader {
    pub(super) id: u32,
    gl: Gl,
}

impl Shader {
    pub fn vert(gl: &Gl, src: &str) -> Self {
        Self::new(gl, src, ShaderType::VertexShader)
    }

    pub fn frag(gl: &Gl, src: &str) -> Self {
        Self::new(gl, src, ShaderType::FragmentShader)
    }

    pub fn geom(gl: &Gl, src: &str) -> Self {
        Self::new(gl, src, ShaderType::GeometryShader)
    }

    fn new(gl: &Gl, src: &str, kind: ShaderType) -> Self {
        let id = gl.create_shader(kind);
        let src = std::ffi::CString::new(src).unwrap();

        gl.shader_source(id, 1, &(src.as_ptr()), null());
        gl.compile_shader(id);
        if gl.get_shader_iv(id, ShaderParameterName::CompileStatus) != 1 {
            let len = gl.get_shader_iv(id, ShaderParameterName::InfoLogLength);
            let log = gl.get_shader_log(id, len);
            println!("{len}: {log}");
        }

        Self { id, gl: gl.clone() }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(self.id);
    }
}
