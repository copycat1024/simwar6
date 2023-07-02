#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8);

impl Color {
    pub const BLACK: Self = Self(0);
    pub const MAROON: Self = Self(1);
    pub const AO: Self = Self(2);
    pub const OLIVE: Self = Self(3);
    pub const NAVY: Self = Self(4);
    pub const PURPLE: Self = Self(5);
    pub const TEAL: Self = Self(6);
    pub const SILVER: Self = Self(7);

    pub const GRAY: Self = Self(8);
    pub const RED: Self = Self(9);
    pub const GREEN: Self = Self(10);
    pub const YELLOW: Self = Self(11);
    pub const BLUE: Self = Self(12);
    pub const FUSHSIA: Self = Self(13);
    pub const CYAN: Self = Self(14);
    pub const WHITE: Self = Self(15);

    pub fn rgb(&self) -> (f32, f32, f32) {
        match self.0 {
            0 => (0., 0., 0.),
            1 => (0.5, 0.25, 0.25),
            2 => (0.25, 0.5, 0.25),
            3 => (0.5, 0.5, 0.25),
            4 => (0.25, 0.25, 0.5),
            5 => (0.5, 0.25, 0.5),
            6 => (0.25, 0.5, 0.5),
            7 => (0.75, 0.75, 0.75),
            8 => (0.5, 0.5, 0.5),
            9 => (1., 0.25, 0.25),
            10 => (0.25, 1., 0.25),
            11 => (1., 1., 0.25),
            12 => (0.25, 0.25, 1.),
            13 => (1., 0.25, 1.),
            14 => (0.25, 1., 1.),
            15 => (1., 1., 1.),

            _ => (0., 0., 0.),
        }
    }
}
