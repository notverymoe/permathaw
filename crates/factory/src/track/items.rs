// Copyright 2024 Natalie Baker // AGPLv3 //

use crate::{
    util::{insert_into, remove_from}, 
    item::ItemStack
};

#[derive(Clone, Copy)]
pub struct TrackBufferLength(u8);

#[derive(Clone, Copy)]
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

    pub fn new(len: &'a mut TrackBufferLength, buffer: &'a mut TrackBuffer) -> Self {
        Self{len: &mut len.0, data: &mut buffer.0}
    }

    pub fn insert(&mut self, idx: usize, v: ItemStack) -> bool {
        let result = insert_into(self.as_slice_mut(), idx, v);
        if result { *self.len += 1; }
        result
    }

    pub fn remove(&mut self, idx: usize) -> Option<ItemStack> {
        let result = remove_from(self.as_slice_mut(), idx);
        if result.is_some() { *self.len -= 1; }
        result
    }

    #[must_use]
    pub fn get(&self, idx: usize) -> Option<ItemStack> {
        self.as_slice().get(idx).copied()
    }

    #[must_use]
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut ItemStack> {
        self.as_slice_mut().get_mut(idx)
    }

    #[must_use]
    pub fn as_slice(&self) -> &[ItemStack] {
        &self.data[..(*self.len as usize)]
    }

    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [ItemStack] {
        &mut self.data[..(*self.len as usize)]
    }

    #[must_use]
    pub fn len(&self) -> usize {
        *self.len as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self.len == 0
    }

}
