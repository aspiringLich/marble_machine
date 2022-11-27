use crate::*;

#[derive(Copy, Clone, Debug)]
pub enum Marble {
    Bit { value: bool },
    Basic { value: u8 },
}
