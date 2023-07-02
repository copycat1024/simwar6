use ab_glyph::{point, Font, FontRef, Point, ScaleFont};

pub struct Text<'a> {
    font: FontRef<'a>,
    scale: f32,
    origin: Point,
}

impl<'a> Text<'a> {
    pub fn new(data: &'a [u8], scale: f32) -> Self {
        let font = FontRef::try_from_slice(data).expect("Cannot load font");
        let ascent = font.as_scaled(scale).ascent();
        let origin = point(0., ascent);

        Self {
            font,
            scale,
            origin,
        }
    }

    pub fn render<F>(&self, c: char, mut callback: F)
    where
        F: FnMut(usize, usize, f32),
    {
        let Self {
            font,
            scale,
            origin,
        } = self;

        let glyph = font.glyph_id(c).with_scale_and_position(*scale, *origin);

        if let Some(outline) = font.outline_glyph(glyph) {
            let min = outline.px_bounds().min;
            outline.draw(|x, y, c| {
                let x = min.x as usize + x as usize;
                let y = min.y as usize + y as usize;
                callback(x, y, c);
            });
        }
    }
}
