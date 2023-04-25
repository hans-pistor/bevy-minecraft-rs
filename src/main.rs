use bevy::prelude::*;
use bevy_minecraft_rs_lib::BevyMinecraftPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(BevyMinecraftPlugin)
        .run();
}
