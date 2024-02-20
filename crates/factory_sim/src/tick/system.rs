// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::{Cooldown, Tick};

pub fn update_cooldowns(q_cooldown: Query<(Entity, &Cooldown)>, tick: Res<Tick>, mut commands: Commands) {
    for (id, cooldown) in &q_cooldown {
        if cooldown.is_done(*tick) {
            commands.entity(id).remove::<Cooldown>();
        }
    }
}