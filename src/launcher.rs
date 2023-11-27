use crate::app::menu;
use soyo::{
    backend::Raito,
    gfx::{Context, Slot},
    mvc::App,
};

pub fn run() {
    let raito = Raito::new();
    let mut ctx = Context::new(raito);

    App::run::<menu::Control, Slot>(&mut (), &mut ctx);
}
