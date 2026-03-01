
const CHUNK_SIZE: usize = 16;
const CHUNK_HEIGHT: usize = 256;
const VIEW_DISTANCE: usize = 4;
const MAX_PLAYERS: usize = 16;
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

pub struct Vector2
{
        x: f32,
        y: f32,
}

pub struct Vector2i
{
        x: i32,
        y: i32,
}

pub struct Vector3
{
        x: f32,
        y: f32,
        z: f32,
}

pub struct Vector3i
{
        x: i32,
        y: i32,
        z: i32,
}

pub struct Chunk
{
        // Using a flat array for performance
        blocks: [BlockType; CHUNK_SIZE*CHUNK_SIZE*CHUNK_HEIGHT],
        xy: Vector2i,
}

pub struct Player<'a>
{
        pos: Vector3,
        vel: Vector3,
        name: Option<&'a str>,
}

pub struct ChunkManager<'a>
{
        chunks: Option<[Chunk; MAX_CHUNKS]>,
        players: Option<[Player<'a>; MAX_PLAYERS]>,
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

impl<'a> Player<'a>
{
        pub fn get_name(&self) -> &Option<&str>
        {
                &self.name
        }

        pub fn set_name(&mut self, new_name: Option<&'a str>)
        {
                self.name = new_name;
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

impl<'a> ChunkManager<'a>
{
        pub fn find_chunk(&self, xy: Vector2i) -> Option<&Chunk>
        {
                match &self.chunks
                {
                        Some(chunks) =>
                        {
                            chunks.iter().find(|chunk| chunk.xy.x == xy.x && chunk.xy.y == xy.y)
                        },
                        None => None,
                }
        }

        pub fn get_chunk(&self, index: usize) -> Option<&Chunk>
        {
                match &self.chunks
                {
                        Some(chunks) =>
                        {
                                if index >= MAX_CHUNKS
                                {
                                        None
                                }
                                else
                                {
                                        Some(&chunks[index])
                                }
                        }
                        None => None,
                }
        }

        pub fn update(&mut self)
        {
                if let Some(chunks) = &self.chunks
                {
                        for chunk in chunks
                        {
                                println!("Test");
                        }
                }
        }

        pub fn new() -> Self
        {
                let default_chunk = Chunk::new(Vector2i::new(0, 0));
                let chunks: [Chunk; MAX_CHUNKS] = std::array::from_fn(|_|
                        {
                        Chunk::new(Vector2i::new(0, 0))
                });
        
                let default_player = Player
                {
                        pos: Vector3::new(0.0, 0.0, 0.0),
                        vel: Vector3::new(0.0, 0.0, 0.0),
                        name: None,
                };

                let players: [Player; MAX_PLAYERS] = std::array::from_fn(|_|
                        {
                        Player
                        {
                                pos: Vector3::new(0.0, 0.0, 0.0),
                                vel: Vector3::new(0.0, 0.0, 0.0),
                                name: None,
                        }
                });
        
                Self
                {
                    chunks: Some(chunks),
                    players: Some(players),
                }
        }
}
