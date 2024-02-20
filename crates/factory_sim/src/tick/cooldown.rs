// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::tick::Tick;

#[derive(Debug, Clone, Copy, Component)]
#[component(storage = "SparseSet")]
pub struct Cooldown(Tick);

impl Cooldown {

    #[must_use]
    pub const fn new(time: Tick, duration: u32) -> Self {
        Self(Tick::new(time.to_raw() + duration))
    }
    
    #[must_use]
    pub const fn is_done(self, time: Tick) -> bool {
        self.0.to_raw() >= time.to_raw()
    }

}
