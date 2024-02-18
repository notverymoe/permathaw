// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::{tick::{PluginTick, TickPacer}, track::PluginTrack};

pub struct PluginsFactory {
    pub pacer: TickPacer,
}

impl PluginGroup for PluginsFactory {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PluginTick::new(self.pacer))
            .add(PluginTrack)
    }
}