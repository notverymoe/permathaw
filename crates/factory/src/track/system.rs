// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::{StackBuffer, TrackBuffer, TrackPassthrough, TrackExtractor, TrackInserter, TrackQueue, TRACK_MAX_ITEMS};

pub fn advance_conveyors(mut q_conveyors: Query<&mut TrackQueue>) {
    for mut conveyor in &mut q_conveyors {
        *conveyor = conveyor.next();
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn handle_track_passthrough(q_connections: Query<(Entity, &TrackPassthrough)>, mut q_conveyors: Query<(&mut TrackQueue, &mut TrackBuffer)>) {
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
            *dst_queue = dst_queue.with(TRACK_MAX_ITEMS);
            let idx = dst_queue.get_buffer_index_of(connection.loc as usize);
            dst_buffer.insert(idx, item).unwrap();
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn handle_track_stack_extractors(mut q_extractors: Query<(&TrackExtractor, &mut StackBuffer)>, mut q_conveyors: Query<(&mut TrackQueue, &mut TrackBuffer)>) {
    for (extractor, mut dst_buffer) in &mut q_extractors {
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
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn handle_track_stack_inserters(mut q_extractors: Query<(&TrackInserter, &mut StackBuffer)>, mut q_conveyors: Query<(&mut TrackQueue, &mut TrackBuffer)>) {
    for (inserter, mut src_buffer) in &mut q_extractors {
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
    }
}
