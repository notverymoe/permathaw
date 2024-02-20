// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::{tick_scheduler, update_cooldowns, PreTick, SubTick1, SubTick2, SubTick3, SubTick4, Tick, TickPacer};

pub struct PluginTick(TickPacer);

impl Default for PluginTick {
    fn default() -> Self {
        Self(TickPacer::unpaced())
    }
}

impl PluginTick {

    #[must_use]
    pub const fn new(pacer: TickPacer) -> Self {
        Self(pacer)
    }

}

impl Plugin for PluginTick {
    fn build(&self, bevy_app: &mut bevy::prelude::App) {
        bevy_app
            .insert_resource(self.0)
            .insert_resource(Tick::new(0))
            .add_schedule(Schedule::new(PreTick))
            .add_schedule(Schedule::new(SubTick1))
            .add_schedule(Schedule::new(SubTick2))
            .add_schedule(Schedule::new(SubTick3))
            .add_schedule(Schedule::new(SubTick4))
            .add_systems(Update, tick_scheduler)
            .add_systems(PreTick, update_cooldowns);
    }
}