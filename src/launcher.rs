use crate::app::{menu, test, ubmp};
use soyo::{
    mvc::{App, Instance, Launcher},
    util::Result,
};

pub type Args = ();

pub const APP_LIST: [(&str, Instance<Args>); 3] = [
    ("Launcher", App::<_, menu::Model>::run),
    ("Test app", App::<_, test::Model>::run),
    ("Unicode plane 0", App::<_, ubmp::Model>::run),
];

pub fn run() -> Result {
    let launcher = Launcher::new(());
    let app_list: Vec<Instance<_>> = APP_LIST.iter().map(|i| i.1).collect();

    launcher.launch(&app_list)
}
