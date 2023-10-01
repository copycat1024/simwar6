use crate::app::{menu, test, ubmp};
use soyo::{
    gfx::{backend::Raito, Context},
    mvc::App,
    util::Result,
};

pub const APP_LIST: [&str; 3] = ["Launcher", "Test app", "Unicode plane 0"];

pub fn run() -> Result {
    let raito = Raito::new();
    let mut ctx = Context::new(raito);
    let mut code = 0;

    loop {
        code = match code {
            0 => App::default().run::<(), usize, menu::Model>(&mut (), &mut ctx)?,
            1 => App::default().run::<(), usize, test::Model>(&mut (), &mut ctx)?,
            2 => App::default().run::<(), usize, ubmp::Model>(&mut (), &mut ctx)?,
            _ => break Ok(()),
        }
    }
}
