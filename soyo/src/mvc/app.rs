use super::{Control, Flow, Model, View};
use crate::gfx::{Context, Fragment};

#[derive(Default)]
pub struct App {
    flow: Flow,
}

impl App {
    pub fn run<C, F>(
        args: &mut <C::Model as Model>::Input,
        ctx: &mut Context<F>,
    ) -> <C::Model as Model>::Output
    where
        C: Control<Frag = F>,
        F: Fragment,
    {
        let mut app = App::default();

        let (mut control, mut model, composer) = C::new(args);
        let mut view = View::new(composer);

        // resize on init
        let (w, h) = ctx.size();
        view.resize(w, h, &mut app.flow);

        // main loop
        'main: loop {
            // handle native events

            while let Some(event) = ctx.event() {
                view.handle_event(event, &mut app.flow);

                control.handle(event, &view.node().widget);

                if let Some(output) = control
                    .dispatch(event, &view.node().widget)
                    .and_then(|e| model.reduce(e))
                {
                    break 'main output;
                }
            }

            control.cache(&model, &mut app.flow);

            if app.flow.draw.get() {
                // update view
                control.update(&mut view.node_mut().widget);

                // compose view
                view.compose();

                // draw
                view.draw(ctx);
            }
        }
    }
}
