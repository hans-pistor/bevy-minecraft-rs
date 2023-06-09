use bevy::{prelude::*, utils::HashSet};

use crate::GameState;

#[derive(Default, Resource)]
pub struct AssetTracker(HashSet<HandleUntyped>);

impl AssetTracker {
    pub fn track(&mut self, handle: HandleUntyped) {
        self.0.insert(handle);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn move_state_to_running_when_all_assets_loaded(
    asset_server: Res<AssetServer>,
    tracked: Res<AssetTracker>,
    mut state: ResMut<NextState<GameState>>,
) {
    match asset_server.get_group_load_state(tracked.0.iter().map(|handle| handle.id())) {
        bevy::asset::LoadState::Loaded => {
            info!(
                "All {} tracked assets have been loaded, setting the game to running",
                tracked.0.len()
            );
            state.set(GameState::Running)
        }
        other => {
            info!("Asset state was: {other:?}")
        }
    }
}

pub struct TrackedAssetPlugin;
impl Plugin for TrackedAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTracker>().add_system(
            move_state_to_running_when_all_assets_loaded
                .in_base_set(CoreSet::Last)
                .run_if(in_state(GameState::Loading)),
        );
    }
}
