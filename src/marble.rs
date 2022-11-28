use crate::*;

#[derive(Copy, Clone, Debug, Component)]
pub enum Marble {
    Bit { value: bool },
    Basic { value: u8 },
}
