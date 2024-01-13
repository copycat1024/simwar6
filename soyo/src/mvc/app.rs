use super::{Control, Model, View};
use crate::gfx::Backend;

pub struct App<C: Control> {
    ctrl: C,
    model: C::Model,
    view: View<C::View>,
}

impl<C: Control> App<C> {
    pub fn new(args: &mut <C::Model as Model>::Input) -> Self {
        let (ctrl, model, composer) = C::new(args);
        let view = View::new(composer);

        Self { ctrl, model, view }
    }

    pub fn run<B>(self, backend: &mut B) -> <C::Model as Model>::Output
    where
        B: Backend<Frag = C::Frag>,
    {
        let Self {
            mut ctrl,
            mut model,
            mut view,
        } = self;

        // resize on init
        let (w, h) = backend.size();
        view.resize(w, h);

        // main loop
        'main: loop {
            // handle native events

            while let Some(event) = backend.event() {
                view.handle_event(event);

                ctrl.handle(event, &view.node().widget);

                if let Some(output) = ctrl
                    .dispatch(event, &view.node().widget)
                    .and_then(|e| model.reduce(e))
                {
                    break 'main output;
                }
            }

            ctrl.cache(&model);
            ctrl.update(&mut view.node_mut().widget);
            view.draw(backend);
        }
    }
}
