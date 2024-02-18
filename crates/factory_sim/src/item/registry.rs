// Copyright 2024 Natalie Baker // AGPLv3 //

use core::{num::NonZeroU16, fmt::Debug};

use bevy::utils::{Entry, HashMap};
use nvm_str_id::SmolStr;

use super::Item;

pub struct ItemRegistration {
    pub owner: String,
    pub id:    Item,
}

pub struct ItemRegistryBuilder {
    lookup: HashMap<SmolStr, ItemRegistration>,
    remaining: u16,
}

impl Default for ItemRegistryBuilder {
    fn default() -> Self {
        Self { 
            lookup: HashMap::default(), 
            remaining: 0x0FFF,
        }
    }
}

pub enum ItemRegistryBuilderError {
    AlreadyRegistered(SmolStr, Item, String),
    ExhaustedIds,
}

impl Debug for ItemRegistryBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyRegistered(arg0, arg1, arg2) => {
                write!(f, "ItemRegistry: Item ({arg0}) is already registered as ({arg1:?}) by ({arg2})")
            },
            Self::ExhaustedIds => write!(f, "ItemRegistry: All IDs exhausted"),
        }
    }
}

impl ItemRegistryBuilder {

    /// Registers an item, as being registered by the given owner, and returns the registered id.
    pub fn register(&mut self, name: SmolStr, owner: String) -> Result<Item, ItemRegistryBuilderError> {
        match self.lookup.entry(name) {
            Entry::Occupied(v) => {
                let entry = v.get();
                Err(ItemRegistryBuilderError::AlreadyRegistered(name, entry.id, entry.owner.clone()))
            },
            Entry::Vacant(v) => {
                if self.remaining == 0 {
                    Err(ItemRegistryBuilderError::ExhaustedIds)
                } else {
                    if self.remaining == 0 { return Err(ItemRegistryBuilderError::ExhaustedIds); }
                    let next = NonZeroU16::new(self.remaining);
                    self.remaining -= 1;
                    match next {
                        Some(next) => Ok(v.insert(ItemRegistration{ id: Item(next), owner, }).id),
                        None => unreachable!()
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn get(&mut self, name: SmolStr) -> Option<&ItemRegistration> {
        self.lookup.get(&name)
    }
    
    #[must_use]
    pub fn build(self) -> ItemRegistry {
        ItemRegistry{
            lookup: self.lookup
        }
    }

}

pub struct ItemRegistry {
    lookup: HashMap<SmolStr, ItemRegistration>,
}

impl ItemRegistry {

    #[must_use]
    pub fn get(&mut self, id: SmolStr) -> Option<&ItemRegistration> {
        self.lookup.get(&id)
    }

}
