// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{
    item::ItemStack, 
    plugin::PluginsFactory, 
    tick::{TickPacer, TickRate1}, 
    track::{StackBuffer, TrackBuffer, TrackExtractor, TrackInserter, TrackPassthrough, TrackQueue, TRACK_MAX_ITEMS}
};

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
    app.add_plugins(PluginsFactory{
        pacer: TickPacer::unpaced(),
    });

    let ent1 = app.world.spawn((queue, buffer_with(stack_1), TickRate1)).id();
    let ent2 = app.world.spawn((queue, buffer_with(stack_2), TickRate1)).id();
    let ent3 = app.world.spawn((queue, buffer_with(stack_1), TickRate1)).id();

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
    app.add_plugins(PluginsFactory{
        pacer: TickPacer::unpaced(),
    });

    let track = app.world.spawn((queue, buffer, TickRate1)).id();
    let _mover = app.world.spawn((
        TrackInserter{
            target: track,
            loc:    TRACK_MAX_ITEMS-2,
            cooldown: 0,
        },
        TrackExtractor{
            target: track,
            loc: 1,
            cooldown: 0,
        },
        StackBuffer{
            contents: None,
        },
        TickRate1
    )).id();

    // TODO finish this test
    for i in 0..=TRACK_MAX_ITEMS {
        println!("{:?}", app.world.get::<TrackQueue>(track).unwrap());

        assert_eq!((i, stack_1), (i, app.world.get::<TrackBuffer>(track).unwrap().get(0).unwrap()));
        assert_eq!((i, stack_1), (i, app.world.get::<TrackBuffer>(track).unwrap().get(1).unwrap()));

        if i != TRACK_MAX_ITEMS { app.update(); }
    }

}