// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::{TrackQueue, TRACK_MAX_ITEMS};

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub struct Connection {
    pub dst: Entity,
    pub loc: u8,
}

impl Connection {

    #[must_use]
    pub const fn new(dst: Entity, into: usize) -> Self {
        Self{dst, loc: (into + 1) as u8}
    }

    #[must_use]
    pub const fn new_passthrough(dst: Entity) -> Self {
        Self{dst, loc: TRACK_MAX_ITEMS as u8}
    }

    #[must_use]
    pub const fn can_transfer(&self, src_queue: &TrackQueue, dst_queue: &TrackQueue) -> bool {
        src_queue.has(0) && !dst_queue.has(self.loc as usize) && (self.loc == 0 || dst_queue.can_advance_idx(self.loc as usize))
    }

}