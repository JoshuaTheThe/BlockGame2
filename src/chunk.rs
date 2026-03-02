
use crate::vector::*;
use crate::renderer::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum BlockType
{
        BlockAir,
        BlockStone,
        BlockGrass,
        BlockDirt,
        BlockSand,
        BlockGravel,
        BlockWater,
        BlockCoalOre,
        BlockIronOre,
        BlockGoldOre,
        BlockDiamondOre,
}

#[derive(Clone, Copy)]
pub struct Chunk
{
        // Using a flat array for performance
        pub blocks: [BlockType; BLOCKS_PER_CHUNK],
        pub xy: Vector2i,
}

impl BlockType
{
        pub fn get_color(block: BlockType) -> Color
        {
                match block
                {
                        BlockType::BlockAir => Color::new(0.0, 0.0, 0.0, 0.0),
                        BlockType::BlockStone => Color::new(0.5, 0.5, 0.5, 1.0),
                        BlockType::BlockGrass => Color::new(0.1, 0.5, 0.2, 1.0),
                        BlockType::BlockDirt => Color::new(0.4, 0.2, 0.0, 1.0),
                        BlockType::BlockSand => Color::new(0.9, 0.8, 0.5, 1.0),
                        BlockType::BlockGravel => Color::new(0.6, 0.6, 0.6, 1.0),
                        BlockType::BlockWater => Color::new(0.0, 0.0, 1.0, 0.7),
                        BlockType::BlockCoalOre => Color::new(0.2, 0.2, 0.2, 1.0),
                        BlockType::BlockIronOre => Color::new(0.8, 0.6, 0.4, 1.0),
                        BlockType::BlockGoldOre => Color::new(1.0, 0.8, 0.2, 1.0),
                        BlockType::BlockDiamondOre => Color::new(0.4, 0.8, 1.0, 1.0),
                }
        }
}

impl Chunk
{
        fn add_block_faces(&self, x: usize, y: usize, z: usize, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>)
        {
                let block_pos = [
                        x as f32 - (CHUNK_SIZE as f32 / 2.0),  // Center chunk at origin
                        y as f32 - (CHUNK_SIZE as f32 / 2.0),
                        z as f32,
                ];
                    
                let face_color: Color = BlockType::get_color(self.blocks[Self::index(x, y, z)]);
                    
                let is_air = |bx: i32, by: i32, bz: i32| -> bool {
                        if bx < 0 || bx >= CHUNK_SIZE as i32 ||
                           by < 0 || by >= CHUNK_SIZE as i32 ||
                           bz < 0 || bz >= CHUNK_HEIGHT as i32 {
                            return true;
                        }
                        match self.blocks[Self::index(bx as usize, by as usize, bz as usize)] {
                            BlockType::BlockAir => true,
                            _ => false,
                        }
                };
                    
                let (x, y, z) = (x as i32, y as i32, z as i32);
                let idx_offset = vertices.len() as u32;

                // Front face (positive Y)
                if is_air(x, y + 1, z)
                {
                        vertices.extend_from_slice(&[
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] + 0.5, block_pos[2] - 0.5, face_color), // Bottom left
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] + 0.5, block_pos[2] - 0.5, face_color), // Bottom right
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] + 0.5, block_pos[2] + 0.5, face_color), // Top right
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] + 0.5, block_pos[2] + 0.5, face_color), // Top left
                        ]);
                        indices.extend_from_slice(&[
                            idx_offset, idx_offset + 1, idx_offset + 2,
                            idx_offset, idx_offset + 2, idx_offset + 3,
                        ]);
                }
                    
                // Back face (negative Y)
                if is_air(x, y - 1, z)
                {
                        let idx_offset = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] - 0.5, block_pos[2] + 0.5, face_color), // Top left
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] - 0.5, block_pos[2] + 0.5, face_color), // Top right
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] - 0.5, block_pos[2] - 0.5, face_color), // Bottom right
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] - 0.5, block_pos[2] - 0.5, face_color), // Bottom left
                        ]);
                        indices.extend_from_slice(&[
                            idx_offset, idx_offset + 1, idx_offset + 2,
                            idx_offset, idx_offset + 2, idx_offset + 3,
                        ]);
                }
                    
                // Left face (negative X)
                if is_air(x - 1, y, z)
                {
                        let idx_offset = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] - 0.5, block_pos[2] + 0.5, face_color), // Top front
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] + 0.5, block_pos[2] + 0.5, face_color), // Top back
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] + 0.5, block_pos[2] - 0.5, face_color), // Bottom back
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] - 0.5, block_pos[2] - 0.5, face_color), // Bottom front
                        ]);
                        indices.extend_from_slice(&[
                            idx_offset, idx_offset + 1, idx_offset + 2,
                            idx_offset, idx_offset + 2, idx_offset + 3,
                        ]);
                }
                    
                // Right face (positive X)
                if is_air(x + 1, y, z)
                {
                        let idx_offset = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] - 0.5, block_pos[2] - 0.5, face_color), // Bottom front
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] + 0.5, block_pos[2] - 0.5, face_color), // Bottom back
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] + 0.5, block_pos[2] + 0.5, face_color), // Top back
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] - 0.5, block_pos[2] + 0.5, face_color), // Top front
                        ]);
                        indices.extend_from_slice(&[
                            idx_offset, idx_offset + 1, idx_offset + 2,
                            idx_offset, idx_offset + 2, idx_offset + 3,
                        ]);
                }
                    
                // Bottom face (negative Z) - remember Z is up!
                if is_air(x, y, z - 1)
                {
                        let idx_offset = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] - 0.5, block_pos[2] - 0.5, face_color), // Bottom left
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] - 0.5, block_pos[2] - 0.5, face_color), // Bottom right
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] + 0.5, block_pos[2] - 0.5, face_color), // Top right
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] + 0.5, block_pos[2] - 0.5, face_color), // Top left
                        ]);
                        indices.extend_from_slice(&[
                            idx_offset, idx_offset + 1, idx_offset + 2,
                            idx_offset, idx_offset + 2, idx_offset + 3,
                        ]);
                }
                    
                // Top face (positive Z)
                if is_air(x, y, z + 1)
                {
                        let idx_offset = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] - 0.5, block_pos[2] + 0.5, face_color), // Bottom left
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] - 0.5, block_pos[2] + 0.5, face_color), // Bottom right
                            Vertex::new(block_pos[0] + 0.5, block_pos[1] + 0.5, block_pos[2] + 0.5, face_color), // Top right
                            Vertex::new(block_pos[0] - 0.5, block_pos[1] + 0.5, block_pos[2] + 0.5, face_color), // Top left
                        ]);
                        indices.extend_from_slice(&[
                            idx_offset, idx_offset + 1, idx_offset + 2,
                            idx_offset, idx_offset + 2, idx_offset + 3,
                        ]);
                }
        }

        pub fn generate_mesh(&self) -> Mesh
        {
                let mut vertices: Vec<Vertex> = Vec::new();
                let mut indices: Vec<u32> = Vec::new();
                    
                for x in 0..CHUNK_SIZE
                {
                        for y in 0..CHUNK_SIZE
                        {
                                for z in 0..CHUNK_HEIGHT
                                {
                                        let block = self.blocks[Self::index(x, y, z)];
                                        match block
                                        {
                                                BlockType::BlockAir => continue,
                                                _ =>
                                                {
                                                        self.add_block_faces(x, y, z, &mut vertices, &mut indices);
                                                }
                                        }
                                }
                        }
                }
                    
                Mesh { vertices, indices }
        }
                
        pub fn index(x: usize, y: usize, z: usize) -> usize
        {
                x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE
        }

        pub fn get_block(&self, xyz: Vector3i) -> Option<BlockType>
        {
                if xyz.x < 0 || xyz.x >= CHUNK_SIZE as i32 ||
                   xyz.y < 0 || xyz.y >= CHUNK_SIZE as i32 ||
                   xyz.z < 0 || xyz.z >= CHUNK_HEIGHT as i32 {
                    return None;
                }
        
                let index = Self::index(xyz.x as usize, xyz.y as usize, xyz.z as usize);
                Some(self.blocks[index])
        }
        
        pub fn new(xy: Vector2i) -> Self
        {
                Self {
                        blocks: [BlockType::BlockAir; BLOCKS_PER_CHUNK],
                        xy,
                }
        }
}
