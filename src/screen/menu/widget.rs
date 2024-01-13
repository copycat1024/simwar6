use super::Handle;
use soyo::{
    gfx::Color,
    raito::Slot,
    util::Frame,
    view::{Compose, Composer, Host, Renderer, Visitor, Widget},
    widget::{Label, ListView},
};

pub const APP_LIST: [&str; 2] = ["Test app", "Unicode plane 0"];

pub struct Menu {
    pub(super) top: Renderer<Label>,
    pub(super) list: Composer<ListView>,
}

impl Menu {}

impl Widget for Menu {
    type Handle<'a> = Handle<'a>;
}

impl Compose for Menu {
    type Frag = Slot;

    fn propagate<V: Visitor<Self::Frag>>(&mut self, v: &mut V) {
        self.top.accept_visitor(v);
        self.list.accept_visitor(v);
    }

    fn compose(&mut self, frame: &mut Frame) {
        self.top.layout(frame.set_h(1).rise_z());
        write!(self.top.handle(), "Launcher");

        self.list.compose(
            frame
                .set_x(frame.w / 3)
                .set_y(5)
                .set_w(frame.w / 3)
                .set_h(frame.h - 11)
                .rise_z(),
        );
        self.list.handle().set_list(APP_LIST.iter());
    }
}

impl Default for Menu {
    fn default() -> Self {
        let mut top: Renderer<Label> = Renderer::default();
        top.handle().set_bg(Color::RED);

        let list: Composer<ListView> = Composer::default();

        Self { top, list }
    }
}
