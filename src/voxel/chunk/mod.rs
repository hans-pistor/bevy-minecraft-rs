use bevy::prelude::UVec3;
use ilattice::prelude::Extent;
use ndshape::{Shape, ConstShape3u32};

use super::Voxel;

// Chunks are going to be just like minecraft: 16x256x16
pub const CHUNK_LENGTH: u32 = 16;
pub const CHUNK_HEIGHT: u32 = 256;

pub type ChunkShape = ConstShape3u32<CHUNK_LENGTH, CHUNK_HEIGHT, CHUNK_LENGTH>;

#[derive(Clone)]
pub struct ChunkBuffer {
    data: Box<[Voxel]>,
    shape: ChunkShape
}


impl ChunkBuffer {
    pub fn new(initial: Voxel) -> ChunkBuffer {
        let shape = ChunkShape {};

        Self {
            data: vec![initial; shape.size() as usize].into_boxed_slice(),
            shape
        }
    }

    pub fn new_empty() -> ChunkBuffer {
        Self::new(Voxel::EMPTY_VOXEL)
    }

    pub fn voxel_at(&self, pos: UVec3) -> &Voxel {
        &self.data[self.shape.linearize(pos.to_array()) as usize]
    }

    pub fn slice(&self) -> &[Voxel] {
        &self.data
    }

    pub fn fill_extent(&mut self, extent: Extent<UVec3>, val: Voxel) {
        ndcopy::fill3(extent.shape.to_array(), val, &mut self.data, &self.shape, extent.minimum.to_array());
    }

    pub fn shape(&self) -> &ChunkShape {
        &self.shape
    }
}
