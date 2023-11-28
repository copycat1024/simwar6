use crate::gfx::Event;

pub trait Fragment: 'static {}

pub trait Backend: 'static {
    type Frag: Fragment;

    fn event(&mut self) -> Option<Event>;
    fn push<I>(&mut self, slots: I)
    where
        I: IntoIterator<Item = Self::Frag>;
    fn draw(&mut self);
    fn size(&self) -> (i32, i32);
}
