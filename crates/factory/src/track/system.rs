// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::{StackBuffer, TrackBuffer, TrackPassthrough, TrackExtractor, TrackInserter, TrackQueue, TRACK_MAX_ITEMS};

pub struct TrackPlugin;

///
/// Current issues:
/// - The connections are N-N, really it should be N-1 or 1-1 (N-1 for mid-conveyor inserts, but maybe we just make that something seperate)
/// 
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_track_stack_extractors,
            handle_track_passthrough,
            advance_conveyors,
            handle_track_stack_inserters,
        ).chain());
    }
}

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

#[cfg(test)]
mod test {

    use bevy::prelude::*;

    use crate::{
        item::ItemStack, 
        track::{StackBuffer, TrackBuffer, TrackPassthrough, TrackExtractor, TrackInserter, TrackQueue, TRACK_MAX_ITEMS}
    };

    use super::TrackPlugin;

    #[test]
    pub fn test_conveyor_loop() {
        let stack_1 = ItemStack::from_raw(1, 1);
        let stack_2 = ItemStack::from_raw(2, 1);
        let queue  = TrackQueue::default().with(1);

        let buffer_with = |stack: ItemStack| {
            let mut buffer = TrackBuffer::default();
            buffer.push(stack).unwrap();
            buffer
        };

        let mut app = App::new();
        app.add_plugins(TrackPlugin);

        let ent1 = app.world.spawn((queue, buffer_with(stack_1))).id();
        let ent2 = app.world.spawn((queue, buffer_with(stack_2))).id();
        let ent3 = app.world.spawn((queue, buffer_with(stack_1))).id();

        app.world.get_entity_mut(ent1).unwrap().insert(TrackPassthrough::new_end_to_end(ent2)); // 1 loops with 2
        app.world.get_entity_mut(ent2).unwrap().insert(TrackPassthrough::new_end_to_end(ent1));
        app.world.get_entity_mut(ent3).unwrap().insert(TrackPassthrough::new_end_to_end(ent3)); // 3 loops with self

        for i in 0..=TRACK_MAX_ITEMS {
            // This will calculate absolute position of item on belt. ie. (60 - n) % 60 -> 1, 0, 59, 58 .. 1
            let expected = TrackQueue::default().with(((TRACK_MAX_ITEMS+1) - i) % TRACK_MAX_ITEMS);
            assert_eq!((i, expected), (i, *app.world.get::<TrackQueue>(ent1).unwrap()));
            assert_eq!((i, expected), (i, *app.world.get::<TrackQueue>(ent2).unwrap()));
            assert_eq!((i, expected), (i, *app.world.get::<TrackQueue>(ent3).unwrap()));
            // Check item didn't get lost/mangled on transfer / advance
            let (stack_a, stack_b) = if i < 2 { (stack_1, stack_2) } else { (stack_2, stack_1) };
            assert_eq!((i, stack_a), (i, app.world.get::<TrackBuffer>(ent1).unwrap().get(0).unwrap()));
            assert_eq!((i, stack_b), (i, app.world.get::<TrackBuffer>(ent2).unwrap().get(0).unwrap()));
            assert_eq!((i, stack_1), (i, app.world.get::<TrackBuffer>(ent3).unwrap().get(0).unwrap()));
            if i != TRACK_MAX_ITEMS { app.update(); }
        }

        assert_eq!(queue, *app.world.get::<TrackQueue>(ent1).unwrap());
        assert_eq!(queue, *app.world.get::<TrackQueue>(ent2).unwrap());
        assert_eq!(queue, *app.world.get::<TrackQueue>(ent3).unwrap());
    }

    #[test]
    pub fn test_self_inserter() {
        let stack_1 = ItemStack::from_raw(1, 1);
        let queue  = TrackQueue::default().with(1).with(3);

        let buffer = {
            let mut buffer = TrackBuffer::default();
            buffer.push(stack_1).unwrap();
            buffer.push(stack_1).unwrap();
            buffer
        };

        let mut app = App::new();
        app.add_plugins(TrackPlugin);

        let track = app.world.spawn((queue, buffer)).id();
        let _mover = app.world.spawn((
            TrackInserter{
                target: track,
                loc:    TRACK_MAX_ITEMS-2
            },
            TrackExtractor{
                target: track,
                loc: 1,
            },
            StackBuffer{
                contents: None,
            }
        )).id();


        for i in 0..=TRACK_MAX_ITEMS {
            println!("{:?}", app.world.get::<TrackQueue>(track).unwrap());

            assert_eq!((i, stack_1), (i, app.world.get::<TrackBuffer>(track).unwrap().get(0).unwrap()));
            assert_eq!((i, stack_1), (i, app.world.get::<TrackBuffer>(track).unwrap().get(1).unwrap()));

            if i != TRACK_MAX_ITEMS { app.update(); }
        }

    }

}
