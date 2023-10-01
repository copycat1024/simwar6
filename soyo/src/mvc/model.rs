use super::Flow;
use crate::{
    gfx::{self, Context},
    util::Result,
    view::Compose,
};

pub trait Model<Input, Output>: 'static + Sized {
    type Event: 'static + Copy;
    type View: Compose;

    fn new(args: &Input) -> (Self, Self::View);
    fn dispatch(&self, event: gfx::Event, view: &Self::View) -> Option<Self::Event>;
    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) -> Option<Output>;

    fn spawn(&self, _ctx: &mut Context) -> Result {
        Ok(())
    }

    fn update(&self, view: &mut Self::View);
}
