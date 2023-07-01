use somme::{Intent, Player};

pub struct Tester {}

impl Player for Tester {
    type Move = Intent;

    fn choose(&self, _list: &[Self::Move]) -> usize {
        0
    }
}
