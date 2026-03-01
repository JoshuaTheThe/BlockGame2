
const CHUNK_SIZE: usize = 16;
const CHUNK_HEIGHT: usize = 256;
const VIEW_DISTANCE: usize = 4;
const MAX_PLAYERS: usize = 16;
const EXTRA_CHUNKS: usize = 1;
const FLOOR_PI: usize = 3;
const MAX_CHUNKS: usize = VIEW_DISTANCE * FLOOR_PI + EXTRA_CHUNKS;
const BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

#[derive(Clone, Copy)]
enum BlockType
{
        BlockAir,
        BlockStone,
}

struct Vector2
{
        x: f32,
        y: f32,
}

struct Vector2i
{
        x: i32,
        y: i32,
}

struct Vector3
{
        x: f32,
        y: f32,
        z: f32,
}

struct Vector3i
{
        x: i32,
        y: i32,
        z: i32,
}

struct Chunk
{
        // Using a flat array for performance
        blocks: [BlockType; CHUNK_SIZE*CHUNK_SIZE*CHUNK_HEIGHT],
        xy: Vector2i,
        present: bool,
}

struct Player<'a>
{
        pos: Vector3,
        vel: Vector3,
        name: Option<&'a str>,
        present: bool,
}

struct ChunkManager<'a>
{
        chunks: [Chunk; MAX_CHUNKS],
        players: [Player<'a>; MAX_PLAYERS],
}

impl Vector2
{
        fn new(x: f32, y: f32) -> Self
        {
                Self { x, y }
        }
}

impl Vector2i
{
        fn new(x: i32, y: i32) -> Self
        {
                Self { x, y }
        }
}

impl Vector3
{
        fn new(x: f32, y: f32, z: f32) -> Self
        {
                Self { x, y, z }
        }
}

impl Vector3i
{
        fn new(x: i32, y: i32, z: i32) -> Self
        {
                Self { x, y, z }
        }
}

impl<'a> Player<'a>
{
        fn get_name(&self) -> &Option<&str>
        {
                &self.name
        }

        fn set_name(&mut self, new_name: Option<&'a str>)
        {
                self.name = new_name;
        }
}

impl Chunk
{
        fn get_block(&self, xyz: Vector3i) -> Option<BlockType>
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

        fn new(xy: Vector2i) -> Self
        {
                Self {
                        blocks: [BlockType::BlockAir; BLOCKS_PER_CHUNK],
                        xy,
                        present: true,
                }
        }
}

impl<'a> ChunkManager<'a>
{
        fn find_chunk(&self, xy: Vector2i) -> Option<&Chunk>
        {
                self.chunks.iter().find(|chunk| chunk.xy.x == xy.x && chunk.xy.y == xy.y)
        }

        fn get_chunk(&self, index: usize) -> Option<&Chunk>
        {
                if index >= MAX_CHUNKS
                {
                        None
                }
                else
                {
                        Some(&self.chunks[index])
                }
        }

        fn update(&mut self)
        {
                for chunk in &self.chunks
                {
                        println!("Test");
                }
        }
}
