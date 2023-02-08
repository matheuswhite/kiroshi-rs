use std::time::Duration;
use crate::vram::Vram;

pub trait Hardware {
    fn sleep_ms(duration: Duration);
}

pub trait Display<const W: usize, const H: usize> {
    fn reset(&self);
    fn refresh(&self, vram: &Vram<W, H>);
}
