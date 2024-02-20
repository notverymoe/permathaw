// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{ecs::query::QueryFilter, prelude::*};

use crate::tick::{Cooldown, Tick};
use super::{StackBuffer, TrackBuffer, TrackExtractor, TrackInserter, TrackPassthrough, TrackQueue};

pub fn advance_conveyors<F: QueryFilter>(mut q_conveyors: Query<&mut TrackQueue, F>) {
    for mut conveyor in &mut q_conveyors {
        *conveyor = conveyor.next();
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn handle_track_passthrough<F: QueryFilter>(q_connections: Query<(Entity, &TrackPassthrough), F>, mut q_conveyors: Query<(&mut TrackQueue, &mut TrackBuffer)>) {
    for (src_ent, connection) in &q_connections {

        let can_transfer = {
            let [(src_queue, _), (dst_queue, _)] = q_conveyors.get_many([src_ent, connection.dst]).unwrap();
            connection.can_transfer(src_queue, dst_queue)
        };

        if can_transfer {
            let item = {
                let (mut src_queue, mut src_buffer) = q_conveyors.get_mut(src_ent).unwrap();
                *src_queue = src_queue.without(0);
                src_buffer.pop().unwrap()
            };

            let (mut dst_queue, mut dst_buffer) = q_conveyors.get_mut(connection.dst).unwrap();
            *dst_queue = dst_queue.with(connection.loc as usize);
            let idx = dst_queue.get_buffer_index_of(connection.loc as usize);
            dst_buffer.insert(idx, item).unwrap();
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn handle_track_stack_extractors<F: QueryFilter>(
    mut q_extractors: Query<(Entity, &TrackExtractor, &mut StackBuffer), (Without<Cooldown>, F)>, 
    mut q_conveyors: Query<(&mut TrackQueue, &mut TrackBuffer)>,
    mut commands: Commands,
    tick: Res<Tick>,
) {
    for (id, extractor, mut dst_buffer) in &mut q_extractors {
        if dst_buffer.contents.is_some() {
            continue;
        }

        let (mut src_queue, mut src_buffer) = q_conveyors.get_mut(extractor.target).unwrap();
        if !src_queue.has(extractor.loc) {
            continue;
        }

        let idx = src_queue.get_buffer_index_of(extractor.loc);
        *src_queue = src_queue.without(extractor.loc);
        dst_buffer.contents = src_buffer.remove(idx);
        if extractor.cooldown > 0 {
            commands.entity(id).insert(Cooldown::new(*tick, extractor.cooldown));
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn handle_track_stack_inserters<F: QueryFilter>(
    mut q_extractors: Query<(Entity, &TrackInserter, &mut StackBuffer), (Without<Cooldown>, F)>, 
    mut q_conveyors: Query<(&mut TrackQueue, &mut TrackBuffer)>,
    mut commands: Commands,
    tick: Res<Tick>,
) {
    for (id, inserter, mut src_buffer) in &mut q_extractors {
        if src_buffer.contents.is_none() {
            continue;
        }

        let (mut dst_queue, mut dst_buffer) = q_conveyors.get_mut(inserter.target).unwrap();
        if dst_queue.has(inserter.loc) {
            continue;
        }

        let idx = dst_queue.get_buffer_index_of(inserter.loc);
        *dst_queue = dst_queue.with(inserter.loc);
        dst_buffer.insert(idx, core::mem::take(&mut src_buffer.contents).unwrap()).unwrap();
        if inserter.cooldown > 0 {
            commands.entity(id).insert(Cooldown::new(*tick, inserter.cooldown));
        }
    }
}
