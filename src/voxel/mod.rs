pub mod chunk;

/// Using a type alias for the backing representation of the block material id so that
/// it's easier to migrate once we need more than 255 block types.
pub type BlockMaterialId = u8;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct Voxel {
    /// The `kind` of block this is. Mapped to the registered id of a block material
    pub block_id: BlockMaterialId,
}

impl Voxel {
    pub const EMPTY_VOXEL: Voxel = Voxel::from_block_id(0);

    pub const fn from_block_id(block_id: BlockMaterialId) -> Self {
        Self { block_id }
    }
}

impl Default for Voxel {
    fn default() -> Self {
        Self::EMPTY_VOXEL
    }
}
