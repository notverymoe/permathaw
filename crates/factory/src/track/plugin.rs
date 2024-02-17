// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::tick::SubTick1;

use super::system::{advance_conveyors, handle_track_passthrough, handle_track_stack_extractors, handle_track_stack_inserters};

pub struct PluginTrack;

impl Plugin for PluginTrack {
    fn build(&self, bevy_app: &mut App) {
        bevy_app
            .add_systems(SubTick1, (
                handle_track_stack_extractors,
                handle_track_passthrough,
                advance_conveyors,
                handle_track_stack_inserters,
            ).chain());
    }
}