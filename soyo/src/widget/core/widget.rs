use super::Common;

pub trait Widget: Sized + 'static {
    type Handle<'a>: HandleOf<'a, Widget = Self>;

    fn handle<'a>(&'a mut self, common: &'a mut Common) -> Self::Handle<'a> {
        Self::Handle::new(self, common)
    }
}

pub trait HandleOf<'a> {
    type Widget;

    fn new(widget: &'a mut Self::Widget, common: &'a mut Common) -> Self;
}
