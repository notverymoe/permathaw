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

use bevy::{ecs::{component::Component, system::Query}, gizmos::gizmos::Gizmos, prelude::Vec2, render::color::Color};
use nvm_factory_sim::track::{TrackBuffer, TrackQueue};

#[derive(Debug, Component)]
pub struct ConveyorPath {
    points: Vec<Vec2>,
    item_len: usize,
    path_len: f32,
}

impl ConveyorPath {

    #[must_use]
    pub fn new(points: Vec<Vec2>, item_len: usize) -> Self {
        let mut path_len = 0.0;
        let mut prev = points[0];
        for &next in &points {
            path_len += prev.distance(next);
            prev = next;
        }

        Self {
            points, 
            item_len,
            path_len,
        }
    }

    #[must_use]
    pub fn get_item_point_on_path(&self, idx: usize) -> Vec2 {
        let normalized   = (idx as f32)/(self.item_len as f32);
        let mut rem  = normalized * self.path_len;
        let mut prev = self.points[0];
        let mut i    = 1;
        loop {
            let next = self.points[i];
            let dist = (next - prev).length();
            if (dist >= rem) || (i+1 == self.points.len()) {
                return prev + ((next - prev)/dist * rem);
            }
            i    += 1;
            rem  -= dist;
            prev  = next;
        }
    }

    #[must_use]
    pub fn points(&self) -> &[Vec2] {
        &self.points
    }

    #[must_use]
    pub const fn item_len(&self) -> usize {
        self.item_len
    }

    #[must_use]
    pub const fn path_len(&self) -> f32 {
        self.path_len
    }

}

#[derive(Debug, Component)]
pub struct Inserter {
    pub point: Vec2,
}

#[allow(clippy::missing_panics_doc)]
pub fn render_debug_conveyors(q_conveyors: Query<(&ConveyorPath, &TrackQueue, &TrackBuffer)>, mut gizmos: Gizmos) {
    for (path, queue, buffer) in &q_conveyors {
        gizmos.linestrip_2d(path.points.iter().copied(), Color::YELLOW);

        for (idx, pos) in queue.iter().enumerate() {
            let stack = buffer.get(idx).unwrap();
            let raw   = stack.to_raw().get();
            let h = 360.0 * (((raw >> 8) ^ (raw & 0xFF)) as f32) / (u16::MAX as f32);
            gizmos.circle_2d(path.get_item_point_on_path(pos), 5.0, Color::hsl(h, 1.0, 0.5));
        }
    }
}

