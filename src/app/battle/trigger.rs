use somme::{Action, ActionKind};

#[derive(Clone, Copy)]
pub struct Trigger {
    pub action: ActionKind,
    pub source: usize,
}

impl Trigger {
    pub fn new(action: &Action) -> Self {
        let kind = action.intent.action;
        let source = action.intent.source;

        Self {
            action: kind,
            source,
        }
    }
}
