use crate::{voxel::BlockMaterialId, GameState};
use bevy::{prelude::*, utils::HashMap};

use super::{RegisterEvent, Registry, RegistryKey, RegistryValue, handle_register_events, asset_tracker::TrackedAssets};

pub type BlockRegistry = Registry<BlockMaterialId, BlockRegistryInfo>;

#[derive(Clone)]
pub struct BlockRegistryInfo {
    pub name: &'static str,
    pub texture_handle: Handle<Image>,

    // TODO: remove once we load textures properly
    pub color: Color,
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

fn register_blocks(asset_server: Res<AssetServer>, mut writer: EventWriter<RegisterEvent<BlockMaterialId, BlockRegistryInfo>>, mut tracker: ResMut<TrackedAssets>) {
    info!("Registering blocks");
    let texture_handle: Handle<Image> = asset_server.load("textures/block/stone.png");

    tracker.track(texture_handle.clone_untyped());

    writer.send(RegisterEvent {
        key: 1,
        val: BlockRegistryInfo {
            name: "black",
            texture_handle,
            color: Color::BLACK,
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
