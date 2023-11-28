use crate::app::menu;
use soyo::{mvc::App, raito::Backend};
use std::time::Duration;

pub fn run() {
    let mut backend = Backend::new(Duration::from_millis(50));

    let app = App::<menu::Control>::new(&mut ());
    app.run(&mut backend);
}
