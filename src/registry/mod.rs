use std::fmt::Debug;
use std::hash::Hash;

use bevy::{
    prelude::*,
    utils::{Entry, HashMap},
};

use self::{block::BlockRegistryPlugin, asset_tracker::TrackedAssetPlugin};

pub mod asset_tracker;
pub mod block;

pub trait RegistryKey: Eq + PartialEq + Hash + Debug + Copy {}
pub trait RegistryValue: Debug + Clone {}

#[derive(Resource)]
pub struct Registry<K: RegistryKey, V: RegistryValue> {
    name: &'static str,
    backing_map: HashMap<K, V>,
}

impl<K: RegistryKey, V: RegistryValue> Registry<K, V> {
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
            None => panic!("[{}] No value found at key {id:?}", self.name),
        }
    }
}

pub struct RegisterEvent<K: RegistryKey, V: RegistryValue> {
    pub key: K,
    pub val: V,
}

fn handle_register_events<
    K: RegistryKey + Send + Sync + 'static,
    V: RegistryValue + Send + Sync + 'static,
>(
    mut registry: ResMut<Registry<K, V>>,
    mut reader: EventReader<RegisterEvent<K, V>>,
) {
    for event in reader.iter() {
        let RegisterEvent { key, val } = event;
        info!("Handling register for {:?}", val);
        registry.register(key.clone(), val.clone());
    }
}

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TrackedAssetPlugin)
            .add_plugin(BlockRegistryPlugin);
    }
}
