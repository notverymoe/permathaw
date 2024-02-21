// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

// // //

pub struct PowerNetwork {
    pub line_inefficiency: u32,
}

// // //

pub struct PowerSource {
    pub network: Entity,
    pub max:     u32,
}

pub struct PowerSourceDrain {
    pub amount: u32,
}

// // //

pub struct PowerSink {
    pub network: Entity,
    pub min:     u16,
    pub max:     u16,    
}

pub struct PowerSinkSupply {
    pub amount: u16,
}

// // //

pub struct PowerLine {
    pub length: u8,
}
