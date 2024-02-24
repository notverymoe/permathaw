// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_factory_dbg::{render_debug_conveyors, ConveyorPath};
use nvm_factory_sim::{item::ItemStack, plugin::PluginsFactory, tick::{TickPacer, TickRate1}, track::{TrackBuffer, TrackPassthrough, TrackQueue}};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PluginsFactory{
            pacer: TickPacer::paced(24.0),
        })
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, render_debug_conveyors)
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn(Camera2dBundle{
        transform: Transform::from_translation(Vec3::new(1.0, 1.0, 0.0)*1080.0/4.0),
        ..Default::default()
    });

    let item_a = ItemStack::from_raw(4096, 2);
    let item_b = ItemStack::from_raw(4096, 2);

    let id_a = commands.spawn((
        TickRate1,
        ConveyorPath::new(
            vec![
                Vec2::ZERO,
                Vec2::X * 60.0 * 10.0,
            ],
            60,
        ),
        TrackQueue::from_occupancy_list([9, 19, 29, 39, 49, 59]),
        {
            let mut buffer = TrackBuffer::default();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer
        }
    )).id();


    let id_b = commands.spawn((
        TickRate1,
        ConveyorPath::new(
            vec![
                Vec2::X * 60.0 * 10.0,
                Vec2::Y * 60.0 * 10.0 + Vec2::X * 60.0 * 10.0,
            ],
            60,
        ),
        TrackQueue::from_occupancy_list([9, 19, 29, 39, 49, 59]),
        {
            let mut buffer = TrackBuffer::default();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer
        }
    )).id();


    let id_c = commands.spawn((
        TickRate1,
        ConveyorPath::new(
            vec![
                Vec2::Y * 60.0 * 10.0 + Vec2::X * 60.0 * 10.0,
                Vec2::Y * 60.0 * 10.0,
            ],
            60,
        ),
        TrackQueue::from_occupancy_list([9, 19, 29, 39, 49, 59]),
        {
            let mut buffer = TrackBuffer::default();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer
        }
    )).id();


    let id_d = commands.spawn((
        TickRate1,
        ConveyorPath::new(
            vec![
                Vec2::Y * 60.0 * 10.0,
                Vec2::ZERO,
            ],
            60,
        ),
        TrackQueue::from_occupancy_list([9, 19, 29, 39, 49, 59]),
        {
            let mut buffer = TrackBuffer::default();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer.push(item_a).unwrap();
            buffer.push(item_b).unwrap();
            buffer
        }
    )).id();

    commands.entity(id_a).insert(TrackPassthrough::new(id_d, 59));
    commands.entity(id_b).insert(TrackPassthrough::new(id_a, 59));
    commands.entity(id_c).insert(TrackPassthrough::new(id_b, 59));
    commands.entity(id_d).insert(TrackPassthrough::new(id_c, 59));
}