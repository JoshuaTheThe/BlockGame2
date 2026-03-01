
use crate::vector::*;
use crate::renderer::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BlockType
{
        BlockAir,
        BlockStone,
}

#[derive(Clone, Copy)]
pub struct Chunk
{
        // Using a flat array for performance
        pub blocks: [BlockType; BLOCKS_PER_CHUNK],
        pub xy: Vector2i,
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
                    
                let face_color: Color = Color {r: 0.7, g: 0.7, b: 0.7, a: 1.0};
                let current_block = BlockType::BlockStone;
                    
                let is_air = |bx: i32, by: i32, bz: i32| -> bool {
                        if bx < 0 || bx >= CHUNK_SIZE as i32 ||
                           by < 0 || by >= CHUNK_SIZE as i32 ||
                           bz < 0 || bz >= CHUNK_HEIGHT as i32 {
                            return true; // out of bounds is air (oh well)
                        }
                        match self.blocks[Self::index(bx as usize, by as usize, bz as usize)] {
                            BlockType::BlockAir => true,
                            BlockType::BlockStone => false,
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
                                                BlockType::BlockStone =>
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

                // Project xyz => flat array
                let x: usize = xyz.x as usize; 
                let y: usize = xyz.y as usize * CHUNK_SIZE;
                let z: usize = xyz.z as usize * CHUNK_SIZE * CHUNK_SIZE;
                let index: usize = x + y + z;
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
