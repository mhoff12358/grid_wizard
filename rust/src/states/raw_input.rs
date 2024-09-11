pub struct TimeTick {
    time: usize,
}

pub enum RawInput {
    TimeTick(TimeTick),
}
