use super::{Control, Model, View};
use crate::gfx::Backend;

pub struct App<C: Control> {
    model: C::Model,
    view: View<C::View>,
}

impl<C: Control> App<C> {
    pub fn new(args: &mut <C::Model as Model>::Input) -> Self {
        let (model, composer) = C::new(args);
        let view = View::new(composer);

        Self { model, view }
    }

    pub fn run<B>(self, backend: &mut B) -> <C::Model as Model>::Output
    where
        B: Backend<Frag = C::Frag>,
    {
        let Self {
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

                if let Some(output) =
                    C::dispatch(event, view.handle()).and_then(|e| model.reduce(e))
                {
                    break 'main output;
                }
            }

            view.draw(backend);
        }
    }
}
