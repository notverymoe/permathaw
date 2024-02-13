// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub struct Connection {
    pub src: Entity,
    pub dst: Entity,
}

