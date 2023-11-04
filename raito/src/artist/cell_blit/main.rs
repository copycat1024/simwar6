use super::Cell;
use crate::{
    gl::{
        texture::{self, FilterMode, TextureData, WrapMode},
        Gl, Program, Texture, Vao,
    },
    Context,
};

const VERT: &str = include_str!("s_vert.glsl");
const GEOM: &str = include_str!("s_geom.glsl");
const FRAG: &str = include_str!("s_frag.glsl");

const GLYPH_SIZE: (usize, usize) = (8, 16);
const TABLE_SIZE: usize = 256;
pub const TEXTURE_SIZE: (usize, usize) = (GLYPH_SIZE.0, GLYPH_SIZE.1 * TABLE_SIZE);

pub struct CellBlit {
    program: Program,
    vao: Vao<Cell>,
    texture: Texture<f32>,
    gl: Gl,
}

impl CellBlit {
    pub fn new(ctx: &Context, data: &TextureData<f32>) -> Self {
        let gl = ctx.gl().clone();

        Self {
            program: Program::new(&gl, VERT, FRAG, Some(GEOM)),
            vao: Vao::new(&gl),
            texture: Self::new_texture(&gl, data),
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

    pub fn set_data(&mut self, data: &TextureData<f32>) {
        let Self { texture, gl, .. } = self;
        *texture = Self::new_texture(gl, data);
    }

    fn new_texture(gl: &Gl, data: &TextureData<f32>) -> Texture<f32> {
        texture::Builder::rectangle(gl)
            .config(|pass| {
                pass.set_wrap_x(WrapMode::ClampToEdge);
                pass.set_wrap_y(WrapMode::ClampToEdge);
                pass.set_filter_mag(FilterMode::Nearest);
                pass.set_filter_min(FilterMode::Nearest);
            })
            .build(&data)
    }
}
