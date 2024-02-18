// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::tick::{SubTick1, SubTick2, SubTick3, SubTick4, TickRate1, TickRate2, TickRate3, TickRate4};

use super::system::{advance_conveyors, handle_track_passthrough, handle_track_stack_extractors, handle_track_stack_inserters};

pub struct PluginTrack;

pub type FilterSubTick1 = Or<(With<TickRate4>, With<TickRate3>, With<TickRate2>, With<TickRate1>)>;
pub type FilterSubTick2 = Or<(With<TickRate4>, With<TickRate3>, With<TickRate2>)>;
pub type FilterSubTick3 = Or<(With<TickRate4>, With<TickRate3>)>;
pub type FilterSubTick4 = With<TickRate4>;

impl Plugin for PluginTrack {
    fn build(&self, bevy_app: &mut App) {
        bevy_app
            .add_systems(SubTick1, (
                handle_track_stack_extractors::<FilterSubTick1>,
                handle_track_passthrough::<FilterSubTick1>,
                advance_conveyors::<FilterSubTick1>,
                handle_track_stack_inserters::<FilterSubTick1>,
            ).chain())
            .add_systems(SubTick2, (
                handle_track_stack_extractors::<FilterSubTick2>,
                handle_track_passthrough::<FilterSubTick2>,
                advance_conveyors::<FilterSubTick2>,
                handle_track_stack_inserters::<FilterSubTick2>,
            ).chain())
            .add_systems(SubTick3, (
                handle_track_stack_extractors::<FilterSubTick3>,
                handle_track_passthrough::<FilterSubTick3>,
                advance_conveyors::<FilterSubTick3>,
                handle_track_stack_inserters::<FilterSubTick3>,
            ).chain())
            .add_systems(SubTick4, (
                handle_track_stack_extractors::<FilterSubTick4>,
                handle_track_passthrough::<FilterSubTick4>,
                advance_conveyors::<FilterSubTick4>,
                handle_track_stack_inserters::<FilterSubTick4>,
            ).chain());
    }
}