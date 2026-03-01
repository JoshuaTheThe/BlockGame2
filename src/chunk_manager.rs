
// notice - we are using the Z=Up Coordinate System

const CHUNK_SIZE: usize = 16;
const CHUNK_HEIGHT: usize = 256;
const VIEW_DISTANCE: usize = 4;
const EXTRA_CHUNKS: usize = 1;
const FLOOR_PI: usize = 3;
const MAX_CHUNKS: usize = VIEW_DISTANCE * FLOOR_PI + EXTRA_CHUNKS;
const BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

#[derive(Clone, Copy)]
pub enum BlockType
{
        BlockAir,
        BlockStone,
}

#[derive(Clone, Copy)]
pub struct Vector2
{
        x: f32,
        y: f32,
}

#[derive(Clone, Copy)]
pub struct Vector2i
{
        x: i32,
        y: i32,
}

#[derive(Clone, Copy)]
pub struct Vector3
{
        x: f32,
        y: f32,
        z: f32,
}

#[derive(Clone, Copy)]
pub struct Vector3i
{
        x: i32,
        y: i32,
        z: i32,
}

#[derive(Clone, Copy)]
pub struct Chunk
{
        // Using a flat array for performance
        blocks: [BlockType; BLOCKS_PER_CHUNK],
        xy: Vector2i,
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
        chunks: [Option<Chunk>; MAX_CHUNKS],
        players: Vec<Player>,
}

impl Vector2
{
        pub fn new(x: f32, y: f32) -> Self
        {
                Self { x, y }
        }
}

impl Vector2i
{
        pub fn new(x: i32, y: i32) -> Self
        {
                Self { x, y }
        }
}

impl Vector3
{
        pub fn new(x: f32, y: f32, z: f32) -> Self
        {
                Self { x, y, z }
        }
}

impl Vector3i
{
        pub fn new(x: i32, y: i32, z: i32) -> Self
        {
                Self { x, y, z }
        }
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
                self.chunks.iter().filter_map(|chunk_opt| chunk_opt.as_ref())
                        .find(|chunk| chunk.xy.x == xy.x && chunk.xy.y == xy.y) 
        }

        pub fn get_chunk(&self, index: usize) -> Option<&Chunk>
        {
                if index >= MAX_CHUNKS
                {
                        None
                }
                else
                {
                        self.chunks[index].as_ref()
                }
        }

        pub fn update(&mut self)
        {
                for chunk_opt in self.chunks.iter_mut()
                {
                        if let Some(chunk) = chunk_opt
                        {
                                println!("Test - chunk at ({}, {})", chunk.xy.x, chunk.xy.y);
                        }
                } 
        }

        pub fn new() -> Self
        {
                const NONE: Option<Chunk> = None;
                Self {
                    chunks: [NONE; MAX_CHUNKS],
                    players: Vec::new(),
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
                                *chunk_opt = Some(chunk);
                                break;
                        }
                }
        }

       // Find chunks that should be loaded based on player positions
        pub fn find_chunks_to_load(&self) -> Vec<Vector2i>
        {
                let mut chunks_to_load = Vec::new();
                let view_dist = VIEW_DISTANCE as i32;
                for player in &self.players
                {
                        let player_chunk_x = (player.pos.x as i32) / CHUNK_SIZE as i32;
                        let player_chunk_y = (player.pos.y as i32) / CHUNK_SIZE as i32;
                        for dx in -view_dist..=view_dist
                        {
                                for dz in -view_dist..=view_dist
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
                chunks_to_load
        }

        // Remove chunks that are too far from all players
        pub fn remove_chunks(&mut self) -> usize
        {
                let mut removed_count = 0;
                let view_distance_sq = (VIEW_DISTANCE * VIEW_DISTANCE) as i32;
                if self.players.is_empty()
                {
                        for chunk_opt in self.chunks.iter_mut()
                        {
                                if chunk_opt.is_some()
                                {
                                        *chunk_opt = None;
                                        removed_count += 1;
                                }
                        }
                        return removed_count;
                }
            
                for chunk_opt in self.chunks.iter_mut() 
                {
                        if let Some(chunk) = chunk_opt
                        {
                                let chunk_x = chunk.xy.x;
                                let chunk_y = chunk.xy.y;
                                let mut keep_chunk = false;                    
                                for player in &self.players
                                {
                                        let player_chunk_x = (player.pos.x as i32) / CHUNK_SIZE as i32;
                                        let player_chunk_y = (player.pos.y as i32) / CHUNK_SIZE as i32;
                                        let dx = player_chunk_x - chunk_x;
                                        let dy = player_chunk_y - chunk_y;
                                        let dist_sq = dx * dx + dy * dy;
                                        if dist_sq <= view_distance_sq
                                        {
                                                keep_chunk = true;
                                                break;
                                        }
                                }
                                if !keep_chunk
                                {
                                        *chunk_opt = None;
                                        removed_count += 1;
                                }
                        }
                }
                removed_count
        }

        pub fn generate(&self, xy: Vector2i) -> Chunk
        {
                Chunk {
                        blocks: [BlockType::BlockStone; BLOCKS_PER_CHUNK],
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
}
