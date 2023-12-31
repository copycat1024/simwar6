use super::Event;
use soyo::mvc;

#[derive(Default)]
pub struct Model {
    pub id: Option<usize>,
}

impl mvc::Model for Model {
    type Event = Event;
    type Input = ();
    type Output = usize;

    fn new(_: &Self::Input) -> Self {
        Model::default()
    }

    fn reduce(&mut self, event: Self::Event) -> Option<usize> {
        match event {
            Self::Event::Exit => Some(usize::MAX),
            Self::Event::StartApp(id) => {
                println!("{id}");
                self.id = Some(id);
                None
            }
        }
    }
}
