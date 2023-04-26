use crate::{voxel::BlockMaterialId, GameState};
use bevy::{prelude::*, utils::HashMap};

use super::{
    asset_tracker::AssetTracker, handle_register_events, RegisterEvent, Registry, RegistryKey,
    RegistryValue,
};

pub type BlockRegistry = Registry<BlockMaterialId, BlockRegistryInfo>;

#[derive(Clone)]
pub struct BlockRegistryInfo {
    pub name: &'static str,
    pub material_handle: Handle<StandardMaterial>,
}

impl std::fmt::Debug for BlockRegistryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockRegistryInfo")
            .field("name", &self.name)
            .finish()
    }
}

impl RegistryKey for BlockMaterialId {}
impl RegistryValue for BlockRegistryInfo {}

fn register_blocks(
    asset_server: Res<AssetServer>,
    mut writer: EventWriter<RegisterEvent<BlockMaterialId, BlockRegistryInfo>>,
    mut tracker: ResMut<AssetTracker>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Registering blocks");
    let texture_handle: Handle<Image> = asset_server.load("textures/block/stone.png");

    tracker.track(texture_handle.clone_untyped());

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..Default::default()
    });

    writer.send(RegisterEvent {
        key: 1,
        val: BlockRegistryInfo {
            name: "stone",
            material_handle,
        },
    });

    let texture_handle: Handle<Image> = asset_server.load("textures/block/dirt.png");

    tracker.track(texture_handle.clone_untyped());

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..Default::default()
    });

    writer.send(RegisterEvent {
        key: 2,
        val: BlockRegistryInfo {
            name: "dirt",
            material_handle,
        },
    });

}

pub struct BlockRegistryPlugin;

impl Plugin for BlockRegistryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RegisterEvent<BlockMaterialId, BlockRegistryInfo>>()
            .insert_resource(Registry {
                name: "Block Registry",
                backing_map: HashMap::<BlockMaterialId, BlockRegistryInfo>::new(),
            })
            // Write events
            .add_system(register_blocks.in_schedule(OnEnter(GameState::Loading)))
            // Read events
            .add_system(handle_register_events::<BlockMaterialId, BlockRegistryInfo>);
    }
}
