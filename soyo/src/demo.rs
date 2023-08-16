use soyo::{
    gfx::{backend::Raito, Color, Context, Event, Quad},
    util::Result,
};

fn main() -> Result {
    {
        let backend = Raito::new();
        let mut ctx = Context::new(backend);
        ctx.clear()?;

        'main: loop {
            if let Some(Event::Key { .. }) = ctx.event()? {
                break 'main;
            }

            let mut rect = Quad::xywh(0, 0, 5, 5);
            ctx.render(rect, 1, |_, letter| {
                letter.c = 'X';
                letter.bg = Color::BLUE;
            });
            rect = Quad::xywh(2, 2, 5, 5);
            ctx.render(rect, 2, |_, letter| {
                letter.c = 'O';
                letter.bg = Color::BLUE;
            });

            ctx.draw()?;
        }
    }

    Ok(())
}
