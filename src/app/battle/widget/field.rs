use super::{formation, Grid, Slot};
use crate::app::battle::Trigger;
use somme::{ActionKind, Unit};
use soyo::{
    tui::Color,
    view::{Compose, Composer, Frame, NodeList, Renderer},
};

pub struct Battlefield {
    grid: Renderer<Grid>,
    slot: [Composer<Slot>; 18],
    pos: Vec<(usize, usize, usize)>,
}

impl Battlefield {}

impl Default for Battlefield {
    fn default() -> Self {
        Self {
            grid: Renderer::new(Grid::new(7, 3, 16, 8)),
            slot: Default::default(),
            pos: Vec::new(),
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
    pub fn update_units(&mut self, units: &[Unit]) {
        for (i, unit) in units.iter().enumerate() {
            let x = unit.pos.x as usize;
            let y = unit.pos.y as usize;
            let team = unit.pos.team;

            if i >= self.pos.len() {
                self.pos.push((x, y, team));
            } else {
                self.pos[i] = (x, y, team);
            }

            self.slot(i).set(|w| w.update(&unit.attr));
        }
    }

    pub fn update_trigger(&mut self, trigger: &Trigger) {
        match trigger.action {
            ActionKind::Attack => {
                self.slot(trigger.source).set(|w| w.attack());
            }
        }
    }

    fn slot(&mut self, i: usize) -> &mut Composer<Slot> {
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
