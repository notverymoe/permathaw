// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::Component;

use crate::item::ItemStack;

pub const BELT_ITEM_SIZE: u8    = 4;
pub const BELT_ITEM_MAX:  usize = 64;
pub const BELT_DIST_MAX:  u8    = ((BELT_ITEM_MAX - 1) as u8) * BELT_ITEM_SIZE;

pub const BELT_BODY_MAX: usize = BELT_ITEM_MAX - 1;

#[derive(Debug, Clone, Copy, Component)]
pub struct BeltHead {
    idx_next: u8,
    idx_prev: u8,
    space: u8,
    count: u8,
}

impl BeltHead {

    pub fn advance_and_squish(&mut self, body: &mut impl AsMut<BeltBody>) {
        if !self.advance_checked() { return; }
        if !self.squish(body.as_mut()) { return; }
        self.advance_unchecked();
    }

    fn advance_checked(&mut self) -> bool {
        // TODO OPT inline always (?)
        let (val, overflowed) = self.space.overflowing_sub(1);
        self.space = val.wrapping_add(overflowed as u8);
        overflowed
    }

    fn advance_unchecked(&mut self) {
        // TODO OPT inline always (?)
        self.space -= 1;
    }

    #[inline(never)]
    fn squish(&mut self, body: &mut BeltBody) -> bool {
        while self.space == 0 {
            if self.idx_next == u8::MAX {
                return false;
            }

            let squish = BeltBodyEntry {
                idx_next: self.idx_prev,
                space: 0,
                stack: body.item_head
            };

            let next = core::mem::replace(&mut body.data[self.idx_next as usize], squish);

            body.item_head = next.stack;
            self.idx_prev  = self.idx_next;
            self.idx_next  = next.idx_next;
            self.space     = next.space;
            self.count    += 1;
        }

        true
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltBodyEntry {
    idx_next: u8,
    space:    u8,
    stack:    ItemStack,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct BeltBody {
    freelist_head: u8,
    data: [BeltBodyEntry; BELT_BODY_MAX],
    item_head: ItemStack,
}

impl BeltBody {

    pub fn remove_after(&mut self, idx_prev: u8) {
        let idx = self.data[idx_prev as usize].idx_next;

        // Next doesn't exist, already free
        if idx as usize >= self.data.len() { return; }

        // Update current to point at removed one's next
        self.data[idx_prev as usize].idx_next = self.data[idx as usize].idx_next;

        // Update freed entry to point to old freelist head and then replace it
        self.data[idx as usize].idx_next = self.freelist_head;
        self.freelist_head = idx;
    }

}