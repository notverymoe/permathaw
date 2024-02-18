// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use super::{Tick, TickPacer};

#[allow(clippy::missing_panics_doc)]
pub fn tick_scheduler(world: &mut World) {
    let pacer = world.get_resource::<TickPacer>().unwrap();

    if pacer.is_paced() {
        let ticks = {
            let delta = world.get_resource::<Time>().unwrap().delta_seconds_f64();
            let mut pacer = world.get_resource_mut::<TickPacer>().unwrap();
            pacer.update(delta)
        };
    
        if ticks == 0 {
            return;
        }
    }

    world.get_resource_mut::<Tick>().unwrap().advance().unwrap();
    world.run_schedule(SubTick1);
    world.run_schedule(SubTick2);
    world.run_schedule(SubTick3);
    world.run_schedule(SubTick4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct SubTick1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct SubTick2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct SubTick3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct SubTick4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TickRate1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TickRate2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TickRate3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TickRate4;
