use super::{formation, Grid, Slot};
use crate::app::battle::Trigger;
use somme::{
    hecs::{Entity, World},
    ActionKind, Attr, Unit,
};
use soyo::{
    gfx::Color,
    view::{Compose, Composer, Frame, NodeList, Renderer},
};
use std::collections::HashMap;

pub struct Battlefield {
    grid: Renderer<Grid>,
    slot: [Composer<Slot>; 18],
    pos: HashMap<Entity, (usize, usize, usize)>,
}

impl Battlefield {}

impl Default for Battlefield {
    fn default() -> Self {
        Self {
            grid: Renderer::new(Grid::new(7, 3, 16, 8)),
            slot: Default::default(),
            pos: HashMap::new(),
        }
    }
}

impl Compose for Battlefield {
    fn register(&mut self, children: &mut NodeList) {
        children.register_renderer(&self.grid);

        for (i, slot) in self.slot.iter_mut().enumerate() {
            slot.set(|w| {
                let (orient, symbol, color) = if i < 9 {
                    (formation::Orient::Right, '>', Color::RED)
                } else {
                    (formation::Orient::Left, '<', Color::BLUE)
                };

                w.set_orient(orient);
                w.set_symbol(symbol);
                w.set_color(color);
            });
            children.register_composer(slot);
        }
    }

    fn layout(&mut self, frame: &mut Frame) {
        use super::GridRenderer;

        let (tw, th) = self.grid.get(|view| view.get_wh());
        self.grid.layout(frame.center(tw, th).rise_z());

        for (i, slot) in self.slot.iter_mut().enumerate() {
            let (x, y) = pos(i);
            slot.layout(self.grid.get_cell(x, y));
        }
    }
}

impl Battlefield {
    pub fn update_units(&mut self, ecs: &World) {
        for (entity, (unit, attr)) in ecs.query::<(&Unit, &mut Attr)>().iter() {
            let x = unit.x as usize;
            let y = unit.y as usize;
            let team = unit.team;

            self.pos.insert(entity, (x, y, team));

            self.slot(&entity).set(|w| w.update(attr));
        }
    }

    pub fn update_trigger(&mut self, trigger: &Trigger) {
        match trigger.action {
            ActionKind::Attack => {
                self.slot(&trigger.source).set(|w| w.attack());
            }
        }
    }

    fn slot(&mut self, i: &Entity) -> &mut Composer<Slot> {
        let (x, y, t) = self.pos[i];
        let i = x + y * 3 + t * 9;
        &mut self.slot[i]
    }
}

fn pos(i: usize) -> (i32, i32) {
    let t = i / 9;
    let y = (i % 9) / 3;
    let x = if t == 0 { i % 3 } else { 2 - i % 3 } + t * 4;
    (x as i32, y as i32)
}
