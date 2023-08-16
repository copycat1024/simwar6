mod raito;
mod render;

pub use self::raito::Raito;

use crate::{
    gfx::{Color, Event, Slot},
    util::Result,
};
use std::time::Duration;

pub trait Backend: 'static {
    fn event(&mut self, event_period: Duration, update_period: Duration) -> Result<Option<Event>>;
    fn print(&mut self, slots: &[Slot]) -> Result;
    fn fg(&mut self, c: Color) -> Result;
    fn bg(&mut self, c: Color) -> Result;
    fn clear(&mut self, color: Color) -> Result;
    fn flush(&mut self) -> Result;
    fn size(&self) -> (i32, i32);
}
