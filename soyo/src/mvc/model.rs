

pub trait Model: 'static + Sized {
    type Event: 'static + Copy;
    type Input;
    type Output;

    fn new(input: &Self::Input) -> Self;
    fn reduce(&mut self, event: Self::Event) -> Option<Self::Output>;
}
