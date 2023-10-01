use super::Flow;
use crate::{gfx, view::Compose};

pub trait Model<Input, Output>: 'static + Sized {
    type Event: 'static + Copy;
    type View: Compose;

    fn new(args: &Input) -> (Self, Self::View);
    fn dispatch(&self, _event: gfx::Event, _view: &Self::View) -> Option<Self::Event>;
    fn reduce(&mut self, _event: Self::Event, _flow: &mut Flow) -> Option<Output>;
    fn update(&self, _view: &mut Self::View);
}
