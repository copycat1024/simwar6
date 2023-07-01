#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlEvent {
    pub tag: u32,
    pub data: [u8; 52],
}

pub const SDL_QUIT: u32 = 256;

impl Default for SdlEvent {
    fn default() -> Self {
        Self {
            tag: 0,
            data: [0; 52],
        }
    }
}

pub enum Event {
    Quit,
}

impl std::convert::TryFrom<SdlEvent> for Event {
    type Error = u32;

    fn try_from(event: SdlEvent) -> Result<Self, u32> {
        let SdlEvent { tag, data } = event;
        match tag {
            SDL_QUIT => Ok(Self::Quit),
            _ => Err(tag),
        }
    }
}
