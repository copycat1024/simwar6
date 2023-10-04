#[derive(Clone, Copy)]
pub enum Event {
    Exit,
    StartApp(usize),
    EndApp,
}
