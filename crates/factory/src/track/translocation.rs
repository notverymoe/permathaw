// Copyright 2024 Natalie Baker // AGPLv3 //

//
// Approaches:
// - Attached to belt.
//   - This would work, but is inconvient and large
//   - Biggest advantage is that all operations for a belt would be applied at once
//   - This can be better parallelized
//

use bevy::prelude::*;

use crate::item::ItemStack;

#[derive(Debug, Clone, Copy, Component)]
pub struct TrackExtractor {
    pub target: Entity,
    pub loc:    usize,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct StackBuffer {
    pub contents: Option<ItemStack>,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct TrackInserter {
    pub target: Entity,
    pub loc:    usize,
}