// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub struct Connection {
    pub src: Entity,
    pub dst: Entity,
}

pub struct ConnectionNew {
    pub sources: SmallVec<[Entity; 4]>,
    pub targets: SmallVec<[    u8; 4]>,
}