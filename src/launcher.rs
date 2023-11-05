use crate::app::menu;
use soyo::{
    gfx::{backend::Raito, Context},
    mvc::App,
};

pub fn run() {
    let raito = Raito::new();
    let mut ctx = Context::new(raito);

    App::run::<menu::Control>(&mut (), &mut ctx);
}
