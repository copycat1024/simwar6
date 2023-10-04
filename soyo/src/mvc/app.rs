use super::{Control, Flow, Model, View};
use crate::{
    gfx::{Context, Event},
    util::Result,
};

#[derive(Default)]
pub struct App {
    flow: Flow,
}

impl App {
    pub fn run<C>(
        mut self,
        args: &mut <C::Model as Model>::Input,
        ctx: &mut Context,
    ) -> Result<<C::Model as Model>::Output>
    where
        C: Control,
    {
        let (mut control, mut model, composer) = C::new(args);
        let mut view = View::new(composer);

        // resize on init
        let (w, h) = ctx.size();
        view.resize(w, h, &mut self.flow)?;

        // main loop
        let output = 'main: loop {
            // handle native events
            while let Some(event) = ctx.event()? {
                match event {
                    Event::Resize { w, h } => {
                        view.resize(w, h, &mut self.flow)?;
                    }
                    Event::Update { delta } => {
                        let delta = delta.as_millis() as u64;
                        view.tick(delta, &mut self.flow);
                    }
                    _ => {}
                }

                control.handle(event, &view.node().widget);
                if let Some(event) = control.dispatch(event, &view.node().widget) {
                    if let Some(output) = model.reduce(event) {
                        break 'main output;
                    }
                }
            }

            control.cache(&model, &mut self.flow);

            if let Some(event) = control.spawn(&model, ctx)? {
                if let Some(output) = model.reduce(event) {
                    break 'main output;
                }
            }

            if self.flow.draw.get() {
                // update view
                control.update(&mut view.node_mut().widget);

                // compose view
                view.compose();

                // draw
                view.draw(ctx)?;
            }
        };

        Ok(output)
    }
}
