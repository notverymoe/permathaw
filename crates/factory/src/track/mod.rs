// Copyright 2024 Natalie Baker // AGPLv3 //

mod queue;
pub use queue::*;

mod items;
pub use items::*;

mod util;

pub struct TrackRef {
    idx:       u32,
    seg_start: u16,
    seg_count: u16,
}

pub struct TrackStorage {
    queue:      Vec<TrackQueue>,
    buffer:     Vec<Vec<TrackBuffer>>,
    buffer_len: Vec<TrackBufferLength>,
}