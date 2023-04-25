use crate::registry::RegistryPlugin;
use crate::render::RenderPlugin;
use bevy::prelude::*;

pub mod registry;
pub mod render;
pub mod voxel;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Running,
}

pub struct BevyMinecraftPlugin;

impl Plugin for BevyMinecraftPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_plugin(RegistryPlugin)
            .add_plugin(RenderPlugin);
    }
}
