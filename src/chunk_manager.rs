
// notice - we are using the Z=Up Coordinate System

use crate::renderer::*;
use crate::vector::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const VIEW_DISTANCE: usize = 2;
pub const EXTRA_CHUNKS: usize = 1;
pub const FLOOR_PI: usize = 3;
pub const MAX_CHUNKS: usize = VIEW_DISTANCE * VIEW_DISTANCE * FLOOR_PI + EXTRA_CHUNKS;
pub const BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

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
        blocks: [BlockType; BLOCKS_PER_CHUNK],
        pub xy: Vector2i,
}

#[derive(Clone)]
pub struct Player
{
        pos: Vector3,
        vel: Vector3,
        name: Option<String>,
}

#[derive(Clone)]
pub struct ChunkManager
{
        chunks: Vec<Option<Box<Chunk>>>,
        players: Vec<Player>,
        meshes: Vec<Mesh>,
}

impl Player
{
        pub fn get_name(&self) -> &Option<String>
        {
                &self.name
        }

        pub fn set_name(&mut self, new_name: Option<String>)
        {
                self.name = new_name;
        }

        pub fn new(name: Option<String>, position: Vector3) -> Self
        {
                Self
                {
                        name: name,
                        pos: position,
                        vel: Vector3::new(0.0, 0.0, 0.0),
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
                
        fn index(x: usize, y: usize, z: usize) -> usize
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

impl ChunkManager
{
        pub fn find_chunk(&self, xy: Vector2i) -> Option<&Chunk>
        {
                self.chunks.iter()
                        .filter_map(|chunk_opt| chunk_opt.as_ref().map(|b| &**b))
                        .find(|chunk| chunk.xy.x == xy.x && chunk.xy.y == xy.y)
        }

        pub fn get_chunk(&self, index: usize) -> Option<&Chunk>
        {
                self.chunks.get(index).and_then(|opt| opt.as_ref().map(|b| &**b))
        }

        pub fn new() -> Self
        {
                const NONE: Option<Chunk> = None;
                Self {
                        chunks: Vec::with_capacity(MAX_CHUNKS),
                        players: Vec::new(),
                        meshes: Vec::new(),
                }
        }

        pub fn add_player(&mut self, initial_pos: Vector3, name: String)
        {
                let player = Player::new(Some(name), initial_pos);
                self.players.push(player);
        }

        pub fn remove_player(&mut self, name: &String)
        {
                self.players.retain(|p| p.name.as_ref() != Some(name));
        }
            
        pub fn get_player(&self, name: &String) -> Option<&Player>
        {
                self.players.iter().find(|p| p.name.as_ref() == Some(name))
        }
            
        pub fn get_player_mut(&mut self, name: &String) -> Option<&mut Player>
        {
                self.players.iter_mut().find(|p| p.name.as_ref() == Some(name))
        }

        pub fn insert_chunk(&mut self, chunk: Chunk)
        {
                for chunk_opt in self.chunks.iter_mut()
                {
                        if chunk_opt.is_none()
                        {
                                *chunk_opt = Some(Box::new(chunk));
                                return;
                        }
                }
                if self.chunks.len() < MAX_CHUNKS
                {
                        self.chunks.push(Some(Box::new(chunk)));
                }
        }

        // Find chunks that should be loaded based on player positions using circular distance
        pub fn find_chunks_to_load(&self) -> Vec<Vector2i>
        {
                let mut chunks_to_load = Vec::new();
                let view_dist = VIEW_DISTANCE as i32;
                let view_dist_squared = (VIEW_DISTANCE * VIEW_DISTANCE) as i32;
                
                for player in &self.players
                {
                        let player_chunk_x = (player.pos.x as i32) / CHUNK_SIZE as i32;
                        let player_chunk_y = (player.pos.y as i32) / CHUNK_SIZE as i32;
                        for dx in -view_dist..=view_dist
                        {
                                for dz in -view_dist..=view_dist
                                {
                                        if dx*dx + dz*dz <= view_dist_squared
                                        {
                                                let chunk_x = player_chunk_x + dx;
                                                let chunk_y = player_chunk_y + dz;
                                                let already_loaded = self.chunks.iter()
                                                        .filter_map(|opt| opt.as_ref())
                                                        .any(|chunk| chunk.xy.x == chunk_x && chunk.xy.y == chunk_y);
                                                if !already_loaded
                                                {
                                                        chunks_to_load.push(Vector2i::new(chunk_x, chunk_y));
                                                }
                                        }
                                }
                        }
                }
                chunks_to_load
        }

        // Remove chunks that are too far from all players
        pub fn remove_chunks(&mut self) -> usize
        {
                let mut removed_count = 0;
                let view_distance_sq = (VIEW_DISTANCE * VIEW_DISTANCE) as i32;
                
                if self.players.is_empty()
                {
                        self.chunks.clear();
                        return 0;
                }
            
                self.chunks.retain_mut(|chunk_opt| {
                        if let Some(chunk) = chunk_opt
                        {
                                let chunk_x = chunk.xy.x;
                                let chunk_y = chunk.xy.y;
                                
                                for player in &self.players
                                {
                                        let player_chunk_x = (player.pos.x as i32) / CHUNK_SIZE as i32;
                                        let player_chunk_y = (player.pos.y as i32) / CHUNK_SIZE as i32;
                                        let dx = player_chunk_x - chunk_x;
                                        let dy = player_chunk_y - chunk_y;
                                        let dist_sq = dx * dx + dy * dy;
                                        if dist_sq <= view_distance_sq
                                        {
                                                return true;
                                        }
                                }
                                removed_count += 1;
                                false
                        }
                        else
                        {
                                false
                        }
                });
                
                println!(" [info] removed {} chunks", removed_count);
                removed_count
        }

        pub fn generate(&self, xy: Vector2i) -> Chunk
        {
                let mut blocks: [BlockType; BLOCKS_PER_CHUNK] = [BlockType::BlockAir; BLOCKS_PER_CHUNK];
                for x in 0..CHUNK_SIZE
                {
                        for y in 0..CHUNK_SIZE
                        {
                                for z in 0..CHUNK_HEIGHT
                                {
                                        if z < 8
                                        {
                                                let index = Chunk::index(x, y, z);
                                                blocks[index] = BlockType::BlockStone;
                                        }
                                }
                        }
                }

                Chunk {
                        blocks: blocks,
                        xy: xy,
                }
        }

        pub fn load_chunks(&mut self)
        {
                let to_load = self.find_chunks_to_load();
                for chunk_xy in to_load
                {
                        if let None = self.find_chunk(chunk_xy)
                        {
                                let chunk = self.generate(chunk_xy);
                                self.insert_chunk(chunk);
                        }
                }
        }

        pub fn generate_meshes(&mut self) -> &[Mesh]
        {
                let mut meshes: Vec<Mesh> = Vec::with_capacity(MAX_CHUNKS);

                for chunk_op in self.chunks.iter()
                {
                        if let Some(chunk) = chunk_op
                        {
                                meshes.push(chunk.generate_mesh());
                        }
                }

                self.meshes = meshes;
                &self.meshes
        }

        pub fn get_meshes(&self) -> &[Mesh]
        {
                &self.meshes
        }

        pub fn needs_mesh_update(&self) -> bool
        {
                let loaded_chunks = self.chunks.iter().filter(|c| c.is_some()).count();
                self.meshes.len() != loaded_chunks
        }
}
