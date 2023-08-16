use soyo::{
    gfx::{backend::Raito, Color, Context, Event, Rect, Slot},
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

            let rect = Rect::xywh(0, 0, 5, 5);
            let iter = rect.iter(true).map(|(x, y)| {
                let mut slot = Slot::new(x, y, 1);
                slot.letter.c = 'X';
                slot.letter.bg = Color::BLUE;
                slot
            });
            ctx.render(iter);

            let rect = Rect::xywh(2, 2, 5, 5);
            let iter = rect.iter(true).map(|(x, y)| {
                let mut slot = Slot::new(x, y, 1);
                slot.letter.c = 'X';
                slot.letter.bg = Color::RED;
                slot
            });
            ctx.render(iter);

            ctx.draw()?;
        }
    }

    Ok(())
}
