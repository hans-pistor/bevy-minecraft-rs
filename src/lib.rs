use bevy::prelude::Plugin;
use registry::RegistryPlugin;

pub mod registry;
pub mod voxel;

pub struct BevyMinecraftPlugin;

impl Plugin for BevyMinecraftPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RegistryPlugin);
    }
}
