use super::Model;
use crate::{
    gfx::{self, Fragment},
    view::{Compose, Widget},
};

pub trait Control: Sized {
    type Frag: Fragment;
    type Model: Model;
    type View: Compose<Frag = Self::Frag>;

    fn new(args: &<Self::Model as Model>::Input) -> (Self::Model, Self::View);

    fn dispatch(
        event: gfx::Event,
        view: <Self::View as Widget>::Handle<'_>,
    ) -> Option<<Self::Model as Model>::Event>;
}
