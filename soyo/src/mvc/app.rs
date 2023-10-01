use super::{Flow, Model, View};
use crate::{
    gfx::{Context, Event},
    util::Result,
};

#[derive(Default)]
pub struct App {
    flow: Flow,
}

impl App {
    pub fn run<I, O, M>(mut self, args: &mut I, ctx: &mut Context) -> Result<O>
    where
        M: Model<I, O>,
    {
        let (mut model, composer) = M::new(args);
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

                if let Some(event) = model.dispatch(event, &view.node().widget) {
                    if let Some(output) = model.reduce(event, &mut self.flow) {
                        break 'main output;
                    }
                }
            }

            if self.flow.draw {
                // update view
                model.update(&mut view.node_mut().widget);

                // compose view
                view.compose();

                // draw
                view.draw(ctx, &mut self.flow)?;
            }
        };

        Ok(output)
    }
}
