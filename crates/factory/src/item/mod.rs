// Copyright 2024 Natalie Baker // AGPLv3 //

use core::num::NonZeroU16;

mod stack;
pub use stack::*;

mod registry;
pub use registry::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item(NonZeroU16);