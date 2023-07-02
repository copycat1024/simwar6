use super::Cell;
use crate::{
    gl::{
        texture::{self, FilterMode, WrapMode},
        Gl, Program, Texture, Vao,
    },
    Context,
};

const VERT: &str = include_str!("s_vert.glsl");
const GEOM: &str = include_str!("s_geom.glsl");
const FRAG: &str = include_str!("s_frag.glsl");

const GLYPH_SIZE: (usize, usize) = (8, 16);
const TABLE_SIZE: usize = 256;

pub struct CellBlit {
    program: Program,
    vao: Vao<Cell>,
    texture: Texture,
    gl: Gl,
}

impl CellBlit {
    pub fn new<F>(ctx: &Context, f: F) -> Self
    where
        F: FnOnce(&mut texture::Pass<f32>),
    {
        let gl = ctx.gl().clone();
        let texture =
            texture::Builder::<f32>::rectangle(&gl, GLYPH_SIZE.0, GLYPH_SIZE.1 * TABLE_SIZE)
                .config(|pass| {
                    pass.set_wrap_x(WrapMode::ClampToEdge);
                    pass.set_wrap_y(WrapMode::ClampToEdge);
                    pass.set_filter_mag(FilterMode::Nearest);
                    pass.set_filter_min(FilterMode::Nearest);

                    f(pass);
                });

        Self {
            program: Program::new(&gl, VERT, FRAG, Some(GEOM)),
            vao: Vao::new(&gl),
            texture: texture.build(),
            gl,
        }
    }

    pub fn draw(&mut self, data: Vec<Cell>) {
        let (wv, hv) = self.gl.get_size();
        let (wc, hc) = GLYPH_SIZE;
        let len = data.len();

        self.vao.config(|pass| {
            pass.vertex().data(data);
        });

        let mut pass = self.program.pass();

        pass.set_2f("view_size", wv as f32, hv as f32);
        pass.set_2f("cell_size", wc as f32, hc as f32);
        pass.bind_texture("tex", &mut self.texture);

        pass.draw(&mut self.vao, len);
    }

    pub fn fg(&mut self, r: f32, g: f32, b: f32) {
        self.program.pass().set_3f("fg", r, g, b);
    }

    pub fn bg(&mut self, r: f32, g: f32, b: f32) {
        self.program.pass().set_3f("bg", r, g, b);
    }
}
