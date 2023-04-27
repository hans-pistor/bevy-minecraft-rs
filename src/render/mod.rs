use bevy::prelude::*;
use ilattice::prelude::Extent;
use ndshape::AbstractShape;

use crate::{
    registry::block::BlockRegistry,
    voxel::{
        chunk::{ChunkBuffer, CHUNK_LENGTH},
        Voxel,
    },
    GameState,
};

fn test_render_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    block_registry: Res<BlockRegistry>,
) {
    info!("Running the startup system.");

    let mut chunk = ChunkBuffer::new_empty();

    // Basically create a plane at the bottom chunk level & fill it in with Voxels w/ id 2
    chunk.fill_extent(
        Extent::from_min_and_shape(
            UVec3::new(0, 0, 0),
            UVec3::new(CHUNK_LENGTH, 1, CHUNK_LENGTH),
        ),
        Voxel::from_block_id(2),
    );

    let chunk_slice = chunk.slice();

    for (index, voxel) in chunk_slice.iter().enumerate() {
        if voxel.block_id == Voxel::EMPTY_VOXEL.block_id {
            // Ignore empty voxels
            continue;
        }
        let [x, y, z] = chunk.shape().delinearize(index as u32);

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.get_handle(&block_registry.get(voxel.block_id).material_handle),
            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
            ..default()
        });
    }

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.get_handle(
            &block_registry
                .get(Voxel::from_block_id(1).block_id)
                .material_handle,
        ),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(test_render_startup_system.in_schedule(OnEnter(GameState::Running)));
    }
}
