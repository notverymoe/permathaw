// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::ItemStack;

pub const BELT_DIST_MAX:  u8 = u8::MAX;
pub const BELT_ITEM_SIZE: u8 = 4;
pub const BELT_ITEM_MAX:  u8 = BELT_DIST_MAX / BELT_ITEM_SIZE;

pub struct BeltTrackEntry {
    next:     u8,
    distance: u8,
    stack:    ItemStack,
}

pub struct BeltTrack {
    data: Vec<BeltTrackEntry>,
    freelist: Vec<u8>,
}
