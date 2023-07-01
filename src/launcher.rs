use crate::app::{battle, menu, test, ubmp};
use somme::Loadout;
use soyo::{
    log::tag,
    mvc::{App, Instance, Launcher},
    util::Result,
};

pub type Args = [Loadout; 10];

pub const APP_LIST: [(&str, Instance<Args>); 4] = [
    ("Launcher", App::<_, menu::Model>::run),
    ("Battle", App::<_, battle::Model>::run),
    ("Test app", App::<_, test::Model>::run),
    ("Unicode plane 0", App::<_, ubmp::Model>::run),
];

pub fn run() -> Result {
    let unit_cfg = [
        Loadout::new("norman", 0, 2, 2),
        Loadout::new("norman", 0, 2, 1),
        Loadout::new("norman", 0, 2, 0),
        Loadout::new("norman", 0, 1, 1),
        Loadout::new("norman", 0, 0, 1),

        Loadout::new("norman", 1, 1, 2),
        Loadout::new("norman", 1, 1, 1),
        Loadout::new("norman", 1, 1, 0),
        Loadout::new("norman", 1, 0, 1),
        Loadout::new("norman", 1, 0, 2),
    ];

    let launcher = Launcher::new(unit_cfg);
    let app_list: Vec<Instance<_>> = APP_LIST.iter().map(|i| i.1).collect();

    launcher.launch(
        &[
            // enable framework logger
            // tag::EVENT,
            tag::DEBUG,
        ],
        &app_list,
    )
}
