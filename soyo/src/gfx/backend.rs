use crate::gfx::{Color, Event};
use std::time::Duration;

pub trait Fragment: 'static {}

pub trait Backend<F: Fragment>: 'static {
    fn event(&mut self, event_period: Duration, update_period: Duration) -> Option<Event>;
    fn push(&mut self, slots: &[F]);
    fn draw(&mut self, color: Color);
    fn size(&self) -> (i32, i32);
}
