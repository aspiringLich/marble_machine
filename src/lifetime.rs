use crate::*;

/// a struct that stores the number of ticks left until it gets borked
#[derive(Deref, DerefMut, Component)]
pub struct Lifetime(usize);
