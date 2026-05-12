// rewritten for performance
// for now it will remain as 32x32x32 non cubic chunks, will convert to cubic soon

use crate::chunk::*;
use crate::player::*;
use crate::renderer::*;
use crate::terrain::*;
use crate::vector::*;
use std::sync::{Arc, RwLock};

pub const VIEW_DISTANCE: usize = 4;
pub const EXTRA_CHUNKS: usize = 32;
pub const FLOOR_PI: usize = 3;
pub const MAX_CHUNKS: usize = VIEW_DISTANCE * VIEW_DISTANCE * FLOOR_PI + EXTRA_CHUNKS;
pub const MAX_TREES: usize = 5;
pub const BEDROCK: usize = 5;
pub const SEA_LEVEL: usize = 25;

#[derive(Clone)]
pub struct ChunkManager
{
        chunks: [Option<Box<Chunk>>; MAX_CHUNKS],
        meshes: Arc<RwLock<[Option<Box<Mesh>>; MAX_CHUNKS]>>,
        chunk_ages: [usize; MAX_CHUNKS], // cache, oldest is overwritten
        players: Vec<Player>,            // this makes sense at least
        noise: Noise3D,
}

impl ChunkManager
{
        pub fn find_chunk(&self, xy: Vector2i) -> Option<&Box<Chunk>>
        {
                self.chunks
                        .iter()
                        .filter_map(|chunk_opt| chunk_opt.as_ref().map(|b| b))
                        .find(|chunk| chunk.xy.x == xy.x && chunk.xy.y == xy.y)
        }

        pub fn get_chunk(&self, index: usize) -> Option<&Box<Chunk>>
        {
                self.chunks.get(index).and_then(|opt| opt.as_ref().map(|b| b))
        }

        pub fn new() -> Self
        {
                const NONE: Option<Chunk> = None;
                Self {
                        chunks: [const { None }; MAX_CHUNKS],
                        meshes: Arc::new(RwLock::new([const { None }; MAX_CHUNKS])),
                        chunk_ages: [256; MAX_CHUNKS],
                        players: Vec::new(),
                        noise: Noise3D { scale: 0.02, seed: 0 },
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

        pub fn victim(&self) -> usize
        {
                let mut victim_idx = 0;
                let mut victim_age = 0;
                for (idx, chunk) in self.chunks.iter().enumerate()
                {
                        if chunk.is_none()
                        {
                                return idx;
                        }

                        if self.chunk_ages[idx] > victim_age
                        {
                                victim_age = self.chunk_ages[idx];
                                victim_idx = idx;
                        }
                }

                victim_idx
        }

        // find a chunk that either does not exist, or is too old
        pub fn insert_chunk(&mut self, chunk: Chunk)
        {
                let victim: usize = self.victim();
                self.chunks[victim] = Some(Box::new(chunk));
                self.chunk_ages[victim] = 0;
                self.chunk_ages.iter_mut().enumerate().for_each(|(i, age)| {
                        if i != victim
                        {
                                *age += 1;
                        }
                });
        }

        pub fn generate_meshes(&mut self)
        {
                for (index, chunk_opt) in self.chunks.iter().enumerate()
                {
                        if let Some(chunk) = chunk_opt
                        {
                                let mesh = Box::new(chunk.generate_mesh());
                                if let Ok(mut meshes) = self.meshes.write()
                                {
                                        meshes[index] = Some(mesh);
                                }
                        }
                }
        }

        pub fn get_meshes(&self) -> &Arc<RwLock<[Option<Box<Mesh>>; MAX_CHUNKS]>>
        {
                &self.meshes
        }

        pub fn load_chunks(&mut self)
        {
                let to_load = self.find_chunks_to_load();
                let mut count: u32 = 0;
                for chunk_xy in to_load
                {
                        if let None = self.find_chunk(chunk_xy)
                        {
                                let chunk = self.generate_noise(chunk_xy);
                                self.insert_chunk(chunk);
                                count += 1;
                        }
                }

                if count > 0
                {
                        self.generate_meshes();
                }
        }

        pub fn find_chunks_to_load(&self) -> Vec<Vector2i>
        {
                let mut chunks_to_load = Vec::new();
                let view_dist = VIEW_DISTANCE as i32;
                let view_dist_squared = (VIEW_DISTANCE * VIEW_DISTANCE) as i32;

                for player in &self.players
                {
                        let player_chunk_x = (player.pos.x as i32) / (CHUNK_SIZE as i32);
                        let player_chunk_y = (player.pos.y as i32) / (CHUNK_SIZE as i32);
                        for dx in -view_dist..=view_dist
                        {
                                for dz in -view_dist..=view_dist
                                {
                                        if dx * dx + dz * dz <= view_dist_squared
                                        {
                                                let chunk_x = player_chunk_x + dx;
                                                let chunk_y = player_chunk_y + dz;
                                                let already_loaded = self
                                                        .chunks
                                                        .iter()
                                                        .filter_map(|opt| opt.as_ref())
                                                        .any(|chunk| {
                                                                chunk.xy.x == chunk_x
                                                                        && chunk.xy.y == chunk_y
                                                        });
                                                if !already_loaded
                                                {
                                                        chunks_to_load.push(Vector2i::new(
                                                                chunk_x, chunk_y,
                                                        ));
                                                }
                                        }
                                }
                        }
                }
                chunks_to_load
        }

        pub fn generate_noise(&self, xy: Vector2i) -> Chunk
        {
                let mut blocks: [BlockType; BLOCKS_PER_CHUNK] =
                        [BlockType::BlockAir; BLOCKS_PER_CHUNK];
                let mut tree_positions: Vec<(i32, i32)> = Vec::new();

                let cave_scale = 0.05;

                for x in 0..CHUNK_SIZE
                {
                        for y in 0..CHUNK_SIZE
                        {
                                let world_x = xy.x * CHUNK_SIZE as i32 + x as i32;
                                let world_y = xy.y * CHUNK_SIZE as i32 + y as i32;

                                for z in 0..CHUNK_SIZE
                                {
                                        let world_z = z as i32;
                                        let nx = world_x as f32;
                                        let ny = world_z as f32;
                                        let nz = world_y as f32;
                                        let mut density =
                                                self.noise.density(nx, ny, nz, 4, 0.5, 2.0);

                                        let height_gradient = world_z as f32 / CHUNK_SIZE as f32;
                                        let target_height = 0.3 + 0.2 * self.noise.density(
                                                world_x as f32 * 0.01,
                                                world_y as f32 * 0.01,
                                                0.0,
                                                3,
                                                0.5,
                                                2.0,
                                        );
                                        density += (target_height - height_gradient) * 0.5;

                                        let cave_noise = self.noise.density(
                                                world_x as f32 * cave_scale,
                                                world_z as f32 * cave_scale * 0.8,
                                                world_y as f32 * cave_scale,
                                                3,
                                                0.6,
                                                2.0,
                                        );

                                        if cave_noise > 0.7 && world_z < 80
                                        {
                                                density -= 0.3;
                                        }

                                        if world_z < 5
                                        {
                                                density += 0.5;
                                        }

                                        let index = Chunk::index(x, y, z);

                                        if density > 0.0
                                        {
                                                blocks[index] = BlockType::BlockStone;
                                                let ore_noise = self.noise.density(
                                                        world_x as f32 * 0.1,
                                                        world_z as f32 * 0.1,
                                                        world_y as f32 * 0.1,
                                                        2,
                                                        0.5,
                                                        2.0,
                                                );

                                                if ore_noise > 0.7
                                                {
                                                        if world_z < 16 && ore_noise > 0.85
                                                        {
                                                                blocks[index] =
                                                                        BlockType::BlockDiamondOre;
                                                        }
                                                        else if world_z < 32 && ore_noise > 0.8
                                                        {
                                                                blocks[index] =
                                                                        BlockType::BlockGoldOre;
                                                        }
                                                        else if world_z < 48 && ore_noise > 0.75
                                                        {
                                                                blocks[index] =
                                                                        BlockType::BlockIronOre;
                                                        }
                                                        else
                                                        {
                                                                blocks[index] =
                                                                        BlockType::BlockCoalOre;
                                                        }
                                                }
                                                else if world_z < 5
                                                {
                                                        blocks[index] = BlockType::BlockBedrock;
                                                }
                                        }
                                }
                        }
                }

                // Second pass: surface decoration
                for x in 0..CHUNK_SIZE
                {
                        for y in 0..CHUNK_SIZE
                        // y is depth
                        {
                                let world_x = xy.x * CHUNK_SIZE as i32 + x as i32;
                                let world_y = xy.y * CHUNK_SIZE as i32 + y as i32; // depth

                                // Find highest solid block (search from top down in z)
                                let mut highest_z = -1;
                                for z in (0..CHUNK_SIZE).rev()
                                {
                                        let index = Chunk::index(x, y, z);
                                        if blocks[index] != BlockType::BlockAir
                                        {
                                                highest_z = z as i32;
                                                break;
                                        }
                                }

                                if highest_z >= 0
                                {
                                        let is_underwater = (highest_z as usize) < SEA_LEVEL;
                                        let index = Chunk::index(x, y, highest_z as usize);

                                        if is_underwater
                                        {
                                                blocks[index] = BlockType::BlockSand;

                                                for d in 1..=3
                                                {
                                                        let below_z = highest_z - d;
                                                        if below_z > 0
                                                        {
                                                                let below_index = Chunk::index(
                                                                        x,
                                                                        y,
                                                                        below_z as usize,
                                                                );
                                                                if blocks[below_index]
                                                                        == BlockType::BlockStone
                                                                {
                                                                        if d <= 2
                                                                        {
                                                                                blocks[below_index] =
                                                                                if rand::random::<u32>() % 3 == 0
                                                                                {
                                                                                        BlockType::BlockGravel
                                                                                }
                                                                                else
                                                                                {
                                                                                        BlockType::BlockSand
                                                                                };
                                                                        }
                                                                        else
                                                                        {
                                                                                blocks[below_index] = BlockType::BlockGravel;
                                                                        }
                                                                }
                                                        }
                                                }
                                        }
                                        else
                                        {
                                                blocks[index] = BlockType::BlockGrass;

                                                // Dirt layers below surface
                                                let dirt_depth =
                                                        2 + (rand::random::<u64>() % 2) as usize;
                                                for d in 1..=dirt_depth
                                                {
                                                        let dirt_z = highest_z - d as i32;
                                                        if dirt_z > 0
                                                        {
                                                                let dirt_index = Chunk::index(
                                                                        x,
                                                                        y,
                                                                        dirt_z as usize,
                                                                );
                                                                if blocks[dirt_index]
                                                                        == BlockType::BlockStone
                                                                {
                                                                        blocks[dirt_index] = BlockType::BlockDirt;
                                                                }
                                                        }
                                                }

                                                // Tree noise - use x and y (depth) for 2D position
                                                let tree_noise = self.noise.density(
                                                        world_x as f32 * 0.1,
                                                        world_y as f32 * 0.1,
                                                        0.0,
                                                        1,
                                                        0.5,
                                                        2.0,
                                                );

                                                if tree_positions.len() < MAX_TREES
                                                        && tree_noise > 0.65
                                                        && highest_z < (CHUNK_SIZE - 5) as i32
                                                        && x >= 2
                                                        && x < CHUNK_SIZE - 2
                                                        && y >= 2
                                                        && y < CHUNK_SIZE - 2
                                                {
                                                        let above_index = Chunk::index(
                                                                x,
                                                                y,
                                                                (highest_z + 1) as usize,
                                                        );
                                                        if (highest_z + 1) < CHUNK_SIZE as i32
                                                                && blocks[above_index]
                                                                        == BlockType::BlockAir
                                                        {
                                                                tree_positions
                                                                        .push((world_x, world_y)); // Store (x, depth)
                                                        }
                                                }
                                        }
                                }
                        }
                }

                // Fill water below sea level
                for x in 0..CHUNK_SIZE
                {
                        for y in 0..CHUNK_SIZE
                        {
                                for z in 0..SEA_LEVEL
                                {
                                        let index = Chunk::index(x, y, z);
                                        if blocks[index] == BlockType::BlockAir
                                        {
                                                blocks[index] = BlockType::BlockWater;
                                        }
                                }
                        }
                }

                Chunk { blocks, xy }
        }
}
