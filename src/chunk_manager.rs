
// notice - we are using the Z=Up Coordinate System

use crate::renderer::*;
use crate::vector::*;
use crate::chunk::*;
use crate::player::*;

pub const VIEW_DISTANCE: usize = 2;
pub const EXTRA_CHUNKS: usize = 1;
pub const FLOOR_PI: usize = 3;
pub const MAX_CHUNKS: usize = VIEW_DISTANCE * VIEW_DISTANCE * FLOOR_PI + EXTRA_CHUNKS;

#[derive(Clone)]
pub struct ChunkManager
{
        chunks: Vec<Option<Box<Chunk>>>,
        players: Vec<Player>,
        meshes: Vec<Mesh>,
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
                                        else if (z < 9)
                                        {
                                                let index = Chunk::index(x, y, z);
                                                blocks[index] = BlockType::BlockGrass;
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
