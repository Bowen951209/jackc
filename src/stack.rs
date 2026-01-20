use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::{instr, instruction::Instruction};

/// U15 = unsigned 15-bit integer (0..32767)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U15(u16);

impl U15 {
    /// Const constructor, will panic at compile time if out of range
    pub const fn new(value: u16) -> Self {
        assert!(value < 1 << 15, "value must fit in 15 bits");
        U15(value)
    }
}

impl FromStr for U15 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(u16::from_str(s)?))
    }
}

impl Display for U15 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rustfmt::skip]
pub fn push_constant(v: U15) -> Box<[Instruction]> {
    instr![
        "@sp",
        "A=M",
        format!("M={v}"),
        "@sp",
        "M=M+1",
    ]
}
