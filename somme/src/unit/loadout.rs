use super::{Attr, Unit};

pub struct Loadout {
    pub unit: Unit,
    pub attr: Attr,
    pub hero: &'static str,
}

impl Loadout {
    pub fn new(hero: &'static str, team: usize, x: i32, y: i32) -> Self {
        Self {
            unit: Unit { hero, team, x, y },
            attr: Attr::new(100, 10),
            hero,
        }
    }
}
