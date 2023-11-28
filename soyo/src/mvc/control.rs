use super::{Flow, Model};
use crate::{
    gfx::{self, Fragment},
    view::Compose,
};

pub trait Control: Sized {
    type Frag: Fragment;
    type Model: Model;
    type View: Compose<Frag = Self::Frag>;

    fn new(args: &<Self::Model as Model>::Input) -> (Self, Self::Model, Self::View);

    fn handle(&mut self, event: gfx::Event, view: &Self::View);
    fn dispatch(
        &mut self,
        event: gfx::Event,
        view: &Self::View,
    ) -> Option<<Self::Model as Model>::Event>;

    fn cache(&mut self, model: &Self::Model, flow: &mut Flow);
    fn update(&self, view: &mut Self::View);
}
