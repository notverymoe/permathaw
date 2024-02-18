// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use super::TRACK_MAX_ITEMS;

use crate::{
    util::{insert_into, remove_from}, 
    item::ItemStack
};

#[derive(Debug, Clone, Copy, Component)]
pub struct TrackBuffer {
    len:   usize,
    items: [ItemStack; TRACK_MAX_ITEMS],
}

impl Default for TrackBuffer {
    fn default() -> Self {
        Self { len: 0, items: [ItemStack::EMPTY; TRACK_MAX_ITEMS] }
    }
}

impl TrackBuffer {

    pub fn push(&mut self, v: ItemStack) -> Result<(), &'static str> {
        self.insert(self.len, v)
    }

    pub fn pop(&mut self) -> Option<ItemStack> {
        self.remove(0)
    }

    pub fn insert(&mut self, idx: usize, v: ItemStack) -> Result<(), &'static str> {
        let result = insert_into(self.as_slice_ext_mut(), idx, v);
        if result.is_ok() { self.len += 1; }
        result
    }

    pub fn remove(&mut self, idx: usize) -> Option<ItemStack> {
        let result = remove_from(self.as_slice_mut(), idx);
        if result.is_some() { self.len -= 1; }
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
        &self.items[..self.len]
    }

    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [ItemStack] {
        &mut self.items[..self.len]
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    fn as_slice_ext_mut(&mut self) -> &mut [ItemStack] {
        if self.len == self.items.len() {
            self.as_slice_mut()
        } else {
            &mut self.items[..=self.len]
        }
    }

}
