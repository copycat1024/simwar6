use crate::app::menu;
use soyo::{
    gfx::{backend::Raito, Context},
    mvc::App,
    util::Result,
};

pub fn run() -> Result {
    let raito = Raito::new();
    let mut ctx = Context::new(raito);

    App::default()
        .run::<menu::Control>(&mut (), &mut ctx)
        .map(|_| ())
}
