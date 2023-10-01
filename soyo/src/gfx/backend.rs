mod raito;

pub use self::raito::Raito;

use crate::{
    gfx::{Color, Event, Slot},
    util::Result,
};
use std::time::Duration;

pub trait Backend: 'static {
    fn event(&mut self, event_period: Duration, update_period: Duration) -> Result<Option<Event>>;
    fn push(&mut self, slots: &[Slot]);
    fn draw(&mut self, color: Color) -> Result;
    fn size(&self) -> (i32, i32);
}
