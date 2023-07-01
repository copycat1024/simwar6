use super::{event::Event, trigger::Trigger, view::View};
use crate::{launcher::Args, simwar::Tester};
use somme::{self, Action, Game, Loadout};
use soyo::{
    mvc::{self, Flow},
    tui::{self, Key},
};

pub struct Model {
    game: Game,
    end: Result<somme::Flow, somme::Error>,
    player: Tester,
}

impl Model {
    fn new(arg: &[Loadout]) -> Self {
        Self {
            game: Game::new(arg).expect("Cannot create game"),
            end: Ok(somme::Flow::Continue),
            player: Tester {},
        }
    }

    pub fn next(&mut self) -> Vec<Action> {
        let mut action_log = Vec::new();
        if let Ok(somme::Flow::Continue) = self.end {
            self.end = self.game.play(&mut self.player).map(|(end, log)| {
                action_log = log;
                end
            });
        }
        action_log
    }
}

impl mvc::Model<Args> for Model {
    type Event = Event;
    type View = View;
    type Trigger = Trigger;

    fn new(arg: &Args) -> (Self, Self::View) {
        (Model::new(arg), View::default())
    }

    fn dispatch(&self, event: tui::Event, _view: &Self::View) -> Option<Self::Event> {
        match event {
            tui::Event::Key { key } => {
                if key == Key::ESC {
                    Some(Event::Exit)
                } else if key == Key(' ') {
                    Some(Event::Next)
                } else {
                    None
                }
            }
            tui::Event::Update { .. } => None,
            _ => None,
        }
    }

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) -> Vec<Self::Trigger> {
        match event {
            Self::Event::Next => {
                let list = self.next();
                flow.draw = true;
                return list.iter().map(Trigger::new).collect();
            }
            Self::Event::Exit => flow.stop = true,
        };
        Vec::new()
    }

    fn trigger(&self, view: &mut Self::View, trigger: Self::Trigger) {
        view.trigger(trigger);
    }

    fn update(&self, view: &mut Self::View) {
        view.update(&self.game);
    }
}
