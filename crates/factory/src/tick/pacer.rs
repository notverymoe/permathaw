// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Resource)]
pub struct TickPacer {
    accum: f64,
    frame: f64,
}

impl TickPacer {
    #[must_use]
    pub fn paced(per_second: f64) -> Self {
        Self {
            accum: 0.0,
            frame: if per_second <= 0.0 { 0.0 } else { 1.0/per_second },
        }
    }

    #[must_use]
    pub const fn unpaced() -> Self {
        Self { accum: 0.0, frame: 0.0 }
    }
}


impl TickPacer {

    pub fn update(&mut self, delta: f64) -> u32 {
        if self.frame == 0.0 { return 1; }
        self.accum += delta;
        let frames = (self.accum/self.frame).floor() as u32;
        self.accum %= self.frame;
        frames
    }

}
