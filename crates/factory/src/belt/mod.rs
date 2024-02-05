// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::ItemStack;

pub const BELT_DIST_MAX:  u8 = u8::MAX;
pub const BELT_ITEM_SIZE: u8 = 4;
pub const BELT_ITEM_MAX:  u8 = BELT_DIST_MAX / BELT_ITEM_SIZE;

pub struct BeltTrack {
    next:      [       u8; BELT_ITEM_MAX as usize],
    distances: [       u8; BELT_ITEM_MAX as usize],
    items:     [ItemStack; BELT_ITEM_MAX as usize],
}
