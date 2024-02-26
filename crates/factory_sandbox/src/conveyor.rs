// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

#[derive(Debug)]
pub struct ConveyorShape {
    points: Vec<Vec2>,
    normals: Vec<Vec2>,

    item_sep:  f32,
    item_size: u32,

    item_length: [u32; 2],
}

impl ConveyorShape {

    #[must_use]
    pub fn new(item_sep: f32, item_size: u32) -> Self {
        Self{
            points:  Vec::default(),
            normals: Vec::default(),
            item_length: [0,0],
            item_sep,
            item_size,
        }
    }

    pub fn push(&mut self, next: Vec2) {
        if let Some(prev) = self.points.last().copied() {
            let delta = next - prev;
            let norm  = delta.normalize().perp();
            let idx = self.normals.len();
            self.normals[idx-1] = self.calc_prev_norm(idx, norm);
            self.normals.push(norm);
        } else {
            self.normals.push(Vec2::ZERO); // We calculate this later
        }

        self.points.push(next);
        self.recalculate_length();
    }

    pub fn set(&mut self, idx: usize, point: Vec2) {
        self.points[idx] = point;
        self.recalculate_normals();
        self.recalculate_length();
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    #[must_use]
    pub fn get_point(&self, idx: usize) -> Vec2 {
        self.points[idx]
    }

    #[must_use]
    pub fn get_normal(&self, idx: usize) -> Vec2 {
        self.normals[idx]
    }

    #[must_use]
    pub fn get_track_points(&self, idx: usize) -> [Vec2; 2] {
        let norm  = self.normals[idx];
        let point = self.points[idx];
        [
            point - norm*self.item_sep,
            point + norm*self.item_sep,
        ]
    }

    fn recalculate_normals(&mut self) {
        for (idx, next) in self.points.iter().copied().enumerate().skip(1) {
            let prev = self.points[idx-1];
            let delta = next - prev;
            let norm = delta.normalize().perp();
            self.normals[idx-1] = self.calc_prev_norm(idx, norm);
            self.normals[idx  ] = norm;
        }
    }

    fn calc_prev_norm(&self, idx: usize, norm: Vec2) -> Vec2 {
        match idx {
            0 => Vec2::ZERO,
            1 => norm,
            _ => (self.normals[idx-1] + norm)/2.0
        }
    }

    fn recalculate_length(&mut self) {
        self.item_length = [0,0];
        for idx in 1..self.len() {
            let prev = self.get_track_points(idx);
            let next = self.get_track_points(idx);
            for i in 0..self.item_length.len() {
                self.item_length[i] = (prev[i].distance(next[i]).floor() as u32)/self.item_size;
            }
        }
    }


}