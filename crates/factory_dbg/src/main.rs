// Copyright 2024 Natalie Baker // AGPLv3 //

#![warn(
    clippy::all, 
    clippy::pedantic,
    clippy::alloc_instead_of_core,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::mixed_read_write_in_expression,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unwrap_in_result,
    clippy::missing_const_for_fn,
)]

#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value, // Bevy systems need this
    // clippy::shadow_unrelated,       // Egui lambda params
)]

use bevy::prelude::*;
use nvm_factory_dbg::{render_debug_conveyors, ConveyorPath};
use nvm_factory_sim::{item::ItemStack, plugin::PluginsFactory, tick::{TickPacer, TickRate1, TickRate4}, track::{TrackBuffer, TrackPassthrough, TrackQueue}};

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
        TickRate4,
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