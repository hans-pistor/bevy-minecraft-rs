use std::fmt::Debug;
use std::hash::Hash;

use bevy::{
    prelude::*,
    utils::{Entry, HashMap},
};

use crate::voxel::BlockMaterialId;

struct BlockRegistryInfo {
    pub name: &'static str,
    pub texture_handle: Handle<Image>,
}

impl Debug for BlockRegistryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockRegistryInfo")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Resource)]
pub struct Registry<K: Eq + PartialEq + Hash + Debug + Copy, V: Debug> {
    name: &'static str,
    backing_map: HashMap<K, V>,
}

impl<K: Eq + PartialEq + Hash + Debug + Copy, V: Debug> Registry<K, V> {
    pub fn register(&mut self, id: K, val: V) {
        match self.backing_map.entry(id) {
            Entry::Occupied(occupied) => panic!(
                "[{}] Tried to register {:?} at id {:?} but {:?} was already registered to that id",
                self.name,
                val,
                id,
                occupied.get()
            ),
            Entry::Vacant(entry) => entry.insert(val),
        };
    }

    pub fn get(&self, id: K) -> &V {
        match self.backing_map.get(&id) {
            Some(info) => info,
            None => panic!("[{}] No value found at key {id:?}", self.name)
        }
    }
}

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Registry {
            name: "Block Registry",
            backing_map: HashMap::<BlockMaterialId, BlockRegistryInfo>::new(),
        });
    }
}
