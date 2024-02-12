// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::Component;

use crate::ItemStack;

#[derive(Clone, Copy, Component)]
pub struct TrackBufferLength(u8);

#[derive(Clone, Copy, Component)]
pub struct TrackBuffer([ItemStack; 64]);

impl TrackBuffer {
    #[must_use]
    pub fn get(&self, len: &TrackBufferLength, idx: usize) -> Option<ItemStack> {
        (idx < len.0 as usize).then(|| self.0[idx])
    }
}


pub struct TrackBufferAccessorMut<'a> {
    len:  &'a mut u8,
    data: &'a mut [ItemStack; 64],
}

impl<'a> TrackBufferAccessorMut<'a> {

    pub fn wrap(len: &'a mut TrackBufferLength, buffer: &'a mut TrackBuffer) -> Self {
        Self{len: &mut len.0, data: &mut buffer.0}
    }

    pub fn remove(&mut self, idx: usize) -> Option<ItemStack> {
        if idx > *self.len as usize {
            return None;
        }
        let result = self.data[idx];
        self.data[idx..].rotate_left(1); // TODO OPT We should just shift

        //unsafe {
        //    core::ptr::copy(self.data.as_mut_ptr().add(idx+1), self.data.as_mut_ptr().add(idx), self.data.len() - 1 - idx);
        //}
        Some(result)
    }

}