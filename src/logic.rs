use crate::interface::{TChannel, TValue};
use crate::trace::Trace;
use std::convert::From;
use std::fmt::{Debug, Formatter, Result};
use std::ops::Index;
use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Copy, Clone, PartialEq)]
pub enum Logic {
    Logic0,
    Logic1,
    Logicz,
    Logicx,
}

impl Logic {
    pub fn is_01(self) -> bool {
        match self {
            Logic::Logic0 | Logic::Logic1 => true,
            _ => false,
        }
    }
}

impl Default for Logic {
    fn default() -> Self {
        Logic::Logicx
    }
}

impl From<bool> for Logic {
    fn from(val: bool) -> Self {
        if val {
            Self::Logic1
        } else {
            Self::Logic0
        }
    }
}

impl From<u8> for Logic {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Logic0,
            1 => Self::Logic1,
            2 => Self::Logicz,
            3 => Self::Logicx,
            _ => panic!(
                "NADJA ERROR: expected `u8` smaller or equal to 3, found {}",
                val
            ),
        }
    }
}

impl From<char> for Logic {
    fn from(val: char) -> Self {
        match val {
            '0' => Self::Logic0,
            '1' => Self::Logic1,
            'z' | 'Z' => Self::Logicz,
            _ => Self::Logicx,
        }
    }
}

impl From<Logic> for bool {
    fn from(val: Logic) -> Self {
        match val {
            Logic::Logic0 => false,
            Logic::Logic1 => true,
            _ => panic!("NADJA ERROR: can not convert {:?} into `bool`", val),
        }
    }
}

impl From<Logic> for char {
    fn from(val: Logic) -> Self {
        match val {
            Logic::Logic0 => '0',
            Logic::Logic1 => '1',
            Logic::Logicz => 'z',
            Logic::Logicx => 'x',
        }
    }
}
impl Not for Logic {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Logic0 => Self::Logic1,
            Self::Logic1 => Self::Logic0,
            _ => Self::Logicx,
        }
    }
}

impl BitOr for Logic {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Logic0, Self::Logic0) => Self::Logic0,
            (Self::Logic1, _) | (_, Self::Logic1) => Self::Logic1,
            _ => Self::Logicx,
        }
    }
}

impl BitAnd for Logic {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Logic1, Self::Logic1) => Self::Logic1,
            (Self::Logic0, _) | (_, Self::Logic0) => Self::Logic0,
            _ => Self::Logicx,
        }
    }
}

impl BitXor for Logic {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Logic0, Self::Logic1) | (Self::Logic1, Self::Logic0) => Self::Logic1,
            (Self::Logic0, Self::Logic0) | (Self::Logic1, Self::Logic1) => Self::Logic0,
            _ => Self::Logicx,
        }
    }
}

impl Debug for Logic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:?}",
            match self {
                Logic::Logic0 => '0',
                Logic::Logic1 => '1',
                Logic::Logicz => 'z',
                Logic::Logicx => 'x',
            }
        )
    }
}

impl Trace for Logic {}
impl TChannel for Logic {}
impl TValue for Logic {}

#[derive(Copy, Clone, PartialEq, Debug, new)]
pub struct VLogic<const WIDTH: usize> {
    val: [Logic; WIDTH],
}

impl<const WIDTH: usize> VLogic<WIDTH> {
    pub fn set(&mut self, val: [Logic; WIDTH]) {
        self.val = val;
    }

    pub fn get(&mut self) -> [Logic; WIDTH] {
        self.val
    }
}
impl<const WIDTH: usize> Trace for VLogic<WIDTH> {}
impl<const WIDTH: usize> TChannel for VLogic<WIDTH> {}
impl<const WIDTH: usize> TValue for VLogic<WIDTH> {}

impl<const WIDTH: usize> Default for VLogic<WIDTH> {
    fn default() -> Self {
        Self {
            val: [Logic::Logicx; WIDTH],
        }
    }
}

impl<const WIDTH: usize> Index<usize> for VLogic<WIDTH> {
    type Output = Logic;
    fn index(&self, index: usize) -> &Self::Output {
        &self.val[index]
    }
}
