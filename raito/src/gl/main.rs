use std::{ffi::CString, ops::Deref, rc::Rc};

pub use super::{
    enums::{GetPName, ProgramPropertyARB, ShaderParameterName},
    GlCmd,
};

pub struct Gl {
    cmd: Rc<GlCmd>,
}

impl Gl {
    pub fn new(cmd: GlCmd) -> Self {
        Self { cmd: Rc::new(cmd) }
    }

    pub fn get_shader_iv(&self, shader: u32, pname: ShaderParameterName) -> i32 {
        get_num(|n| self.cmd.get_shaderiv(shader, pname, n))
    }

    pub fn get_shader_log(&self, shader: u32, size: i32) -> String {
        get_str(size, |n, buf| {
            self.cmd.get_shader_info_log(shader, size, n, buf)
        })
    }

    pub fn get_program_iv(&self, program: u32, pname: ProgramPropertyARB) -> i32 {
        get_num(|n| self.cmd.get_programiv(program, pname, n))
    }

    pub fn get_program_log(&self, program: u32, size: i32) -> String {
        get_str(size, |n, buf| {
            self.cmd.get_program_info_log(program, size, n, buf)
        })
    }

    pub fn new_vbo(&self) -> u32 {
        get_num(|n| self.cmd.gen_buffers(1, n))
    }

    pub fn new_vao(&self) -> u32 {
        get_num(|n| self.cmd.gen_vertex_arrays(1, n))
    }

    pub fn new_texture(&self) -> u32 {
        get_num(|n| self.cmd.gen_textures(1, n))
    }

    pub fn get_uniform_id(&self, program: u32, name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        self.cmd.get_uniform_location(program, name.as_ptr())
    }

    pub fn get_size(&self) -> (i32, i32) {
        let mut size: [i32; 4] = [0; 4];
        self.cmd.get_integerv(GetPName::Viewport, size.as_mut_ptr());
        (size[2], size[3])
    }
}

impl Deref for Gl {
    type Target = GlCmd;

    fn deref(&self) -> &Self::Target {
        &self.cmd
    }
}

impl Clone for Gl {
    fn clone(&self) -> Self {
        Self {
            cmd: self.cmd.clone(),
        }
    }
}

fn get_num<N, F>(f: F) -> N
where
    F: FnOnce(*mut N),
    N: Default,
{
    let mut n = N::default();
    f(&mut n);
    n
}

fn get_str<F>(mut size: i32, f: F) -> String
where
    F: FnOnce(*mut i32, *mut i8),
{
    let mut vec: Vec<u8> = vec![];
    vec.resize(size as usize, 0);
    f(&mut size, vec.as_mut_ptr().cast());
    String::from_utf8(vec).unwrap()
}
