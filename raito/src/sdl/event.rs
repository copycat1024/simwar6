#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlEvent {
    pub tag: u32,
    pub data: [u8; 52],
}

pub const SDL_QUIT: u32 = 256;
pub const SDL_KEY_UP: u32 = 769;
pub const SDL_KEY_DOWN: u32 = 768;

impl Default for SdlEvent {
    fn default() -> Self {
        Self {
            tag: 0,
            data: [0; 52],
        }
    }
}

#[non_exhaustive]
pub enum Event {
    Quit {
        time: u32,
    },
    Key {
        down: bool,
        time: u32,
        window_id: u32,
        state: u8,
        repeat: u8,
        pad1: u16,
        scancode: i32,
        keycode: i32,
        modifier: u16,
    },
}

impl Event {
    fn quit(data: [u8; 52]) -> Self {
        let mut data = Serializer::new(data);

        Self::Quit { time: data.u32() }
    }

    fn key(tag: u32, data: [u8; 52]) -> Self {
        let mut data = Serializer::new(data);

        Self::Key {
            down: tag == SDL_KEY_DOWN,
            time: data.u32(),
            window_id: data.u32(),
            state: data.u8(),
            repeat: data.u8(),
            pad1: data.u16(),

            scancode: data.i32(),
            keycode: data.i32(),

            modifier: data.u16(),
        }
    }
}

impl std::convert::TryFrom<SdlEvent> for Event {
    type Error = u32;

    fn try_from(event: SdlEvent) -> Result<Self, u32> {
        let SdlEvent { tag, data } = event;
        match tag {
            SDL_QUIT => Ok(Self::quit(data)),
            tag @ (SDL_KEY_UP | SDL_KEY_DOWN) => Ok(Self::key(tag, data)),
            _ => Err(tag),
        }
    }
}

struct Serializer {
    iter: std::array::IntoIter<u8, 52>,
}

impl Serializer {
    fn new(data: [u8; 52]) -> Self {
        Self {
            iter: data.into_iter(),
        }
    }

    fn take(&mut self, size: usize) -> u64 {
        self.iter
            .by_ref()
            .take(size)
            .enumerate()
            .fold(0, |c, (i, n)| c + (n as u64).wrapping_shl(i as u32 * 8))
    }

    fn u8(&mut self) -> u8 {
        self.take(1) as u8
    }

    fn u16(&mut self) -> u16 {
        self.take(2) as u16
    }

    fn u32(&mut self) -> u32 {
        self.take(4) as u32
    }

    fn i32(&mut self) -> i32 {
        unsafe { std::mem::transmute(self.take(4) as u32) }
    }
}
