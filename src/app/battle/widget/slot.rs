use super::{formation, Formation, PercentBar};
use crate::util::Timer;
use somme::Attr;
use soyo::{
    tui::Color,
    util::HAlign,
    view::{Compose, Frame, NodeList, Renderer},
    widget::Label,
};

#[derive(Default)]
pub struct Slot {
    formation: Renderer<Formation>,
    name: Renderer<Label>,
    health: Renderer<PercentBar>,
    attack_timer: Timer,
}

impl Slot {
    pub fn attack(&mut self) {
        self.attack_timer.set();
        self.formation
            .set(|w| w.set_status(formation::Status::Strike))
    }

    pub fn update(&mut self, attr: &Attr) {
        let chp: i32 = attr.get("hp").into();
        let mhp = attr.get("max_hp");

        self.name.set(|w| write!(w, "{}", chp));
        self.health.set(|w| w.update(chp, mhp));
        self.formation.set(|w| w.update(chp, mhp));
    }

    pub fn set_orient(&mut self, orient: formation::Orient) {
        self.formation.set(|w| w.set_orient(orient));
    }

    pub fn set_symbol(&mut self, symbol: char) {
        self.formation.set(|w| w.set_symbol(symbol));
    }

    pub fn set_color(&mut self, color: Color) {
        self.health.attr(|a| a.fg = color);
    }
}

impl Compose for Slot {
    fn register(&mut self, children: &mut NodeList) {
        self.attack_timer.t0 = 100;

        self.name.set(|w| {
            write!(w, "Soyo");
            w.set_align(HAlign::Left)
        });
        self.name.attr(|a| a.bg = Color::BLACK);
        self.health.attr(|a| a.bg = Color::BLACK);

        children.register_renderer(&self.name);
        children.register_renderer(&self.health);
        children.register_renderer(&self.formation);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.name.layout(frame.margin(0, 0, 1, 1).set_h(1));
        self.health.layout(frame.margin(1, 0, 0, 0).set_h(1));
        self.formation.layout(frame.margin(2, 0, 0, 0));
    }

    fn tick(&mut self, delta: u64) -> bool {
        if self.attack_timer.tick(delta) {
            self.formation
                .set(|w| w.set_status(formation::Status::Idle));
            true
        } else {
            false
        }
    }
}
