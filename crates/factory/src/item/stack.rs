// Copyright 2024 Natalie Baker // AGPLv3 //

use core::num::NonZeroU16;

use super::Item;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack(NonZeroU16);

impl ItemStack {

    pub const EMPTY: ItemStack = ItemStack(NonZeroU16::MIN);

    #[must_use]
    pub const fn new(item: Item, qty: u16) -> Self {
        assert!(qty & 0x0F == qty);
        if let Some(v) = NonZeroU16::new(item.0.get() | qty << 12) {
            Self(v)
        } else {
            panic!("Could not construct item stack. This isn't possible");
        }
    }

}