// Copyright 2024 Natalie Baker // AGPLv3 //

use core::num::NonZeroU16;

mod stack;
pub use stack::*;

mod registry;
pub use registry::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item(NonZeroU16);

impl Item {

    #[must_use]
    pub const fn as_stack(self, count: usize) -> ItemStack {
        ItemStack::new(self, count)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub const fn from_stack(other: ItemStack) -> Item {
        if let Some(other) = NonZeroU16::new(other.to_raw().get() & 0x0FFF) {
            Self(other)
        } else {
            panic!("Couldn't extract item from stack");
        }
    }

    #[must_use]
    pub const fn to_raw(self) -> NonZeroU16 {
        self.0
    }

}