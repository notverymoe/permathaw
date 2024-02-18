// Copyright 2024 Natalie Baker // AGPLv3 //

use core::num::NonZeroU16;

use super::Item;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack(NonZeroU16);

impl ItemStack {

    pub const EMPTY: ItemStack = ItemStack(NonZeroU16::MIN);

    /// # Panics
    /// - If size >= 16
    #[must_use]
    pub const fn new(item: Item, size: usize) -> Self {
        assert!(size & 0x0F == size);
        if let Some(v) = NonZeroU16::new(item.0.get() | (size as u16) << 12) {
            Self(v)
        } else {
            panic!("Couldn't construct item stack. This isn't possible");
        }
    }

    #[must_use]
    pub const fn item(self) -> Item {
        Item::from_stack(self)
    }

    #[must_use]
    pub const fn size(self) -> usize {
        (self.0.get() >> 12) as usize
    }

    #[must_use]
    pub const fn to_raw(self) -> NonZeroU16 {
        self.0
    }
}

#[cfg(test)]
impl ItemStack {

    /// # Panics
    /// - If amount >= 16
    #[must_use]
    pub const fn from_raw(item: u16, size: usize) -> Self {
        assert!(size & 0x0F == size);
        if let Some(v) = NonZeroU16::new(item | (size as u16) << 12) {
            Self(v)
        } else {
            panic!("Couldn't construct item stack. This isn't possible");
        }
    }

}