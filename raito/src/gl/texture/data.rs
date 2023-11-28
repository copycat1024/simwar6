use super::Pixel;

pub struct TextureData<T: Pixel> {
    data: Vec<T>,
    w: usize,
    h: usize,
}

impl<T: Pixel> TextureData<T> {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            data: vec![T::default(); w * h],
            w,
            h,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, item: T) {
        let Self { data, w, h } = self;
        if x < *w && y < *h {
            data[x + y * *w] = item;
        }
    }

    pub fn get_w(&self) -> usize {
        self.w
    }

    pub fn get_h(&self) -> usize {
        self.h
    }

    pub fn ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn slice(&self) -> &[T] {
        &self.data
    }
}

impl TextureData<f32> {
    pub fn from_u8(data: &[u8], w: usize, h: usize) -> Option<Self> {
        (data.len() == w * h).then_some(Self {
            data: data.iter().map(|n| (*n as f32) / 256.).collect(),
            w,
            h,
        })
    }

    pub fn to_u8(&self) -> Vec<u8> {
        self.data.iter().map(|n| (n * 256.) as u8).collect()
    }
}
