use crate::util::Latch;

#[derive(Default)]
pub struct Flow {
    pub draw: Latch,
    pub spawn: Latch,
}
