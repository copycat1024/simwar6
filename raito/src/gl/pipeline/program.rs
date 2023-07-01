use super::ProgramPass;
use crate::gl::{enums::ProgramPropertyARB, Gl, Shader};

pub struct Program {
    pub(super) id: u32,
    gl: Gl,
}

impl Program {
    pub fn new(gl: &Gl, vert: &str, frag: &str, geom: Option<&str>) -> Self {
        let id = gl.create_program();

        let vert = Shader::vert(gl, vert);
        gl.attach_shader(id, vert.id);

        if let Some(geom) = geom {
            let geom = Shader::geom(gl, geom);
            gl.attach_shader(id, geom.id);
        }

        let frag = Shader::frag(gl, frag);
        gl.attach_shader(id, frag.id);

        gl.link_program(id);
        if gl.get_program_iv(id, ProgramPropertyARB::LinkStatus) != 1 {
            let len = gl.get_program_iv(id, ProgramPropertyARB::InfoLogLength);
            let log = gl.get_program_log(id, len);
            println!("{len}: {log}");
        }

        Self { id, gl: gl.clone() }
    }

    pub(super) fn bind(&self) {
        self.gl.use_program(self.id);
    }

    pub(super) fn unbind(&self) {
        self.gl.use_program(0);
    }

    pub fn pass(&mut self) -> ProgramPass {
        let gl = self.gl.clone();
        ProgramPass::new(self, gl)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(self.id);
    }
}
