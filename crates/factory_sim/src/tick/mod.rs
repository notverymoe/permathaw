// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

mod pacer;
pub use pacer::*;

mod plugin;
pub use plugin::*;

mod schedule;
pub use schedule::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Resource)]
pub struct Tick(u32);

impl Tick {

    #[must_use]
    pub const fn new(tick: u32) -> Self {
        Self(tick)
    }

    pub fn advance(&mut self) -> Result<(), &'static str> {
        if let Some(v) = self.0.checked_add(1) {
            self.0 = v;
            Ok(())
        } else {
            Err("Overflow on tick advance.")
        }
    }

}