#[derive(Clone, Copy)]
pub struct Vector2
{
        pub x: f32,
        pub y: f32,
}

#[derive(Clone, Copy)]
pub struct Vector2i
{
        pub x: i32,
        pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Vector3
{
        pub x: f32,
        pub y: f32,
        pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Vector3i
{
        pub x: i32,
        pub y: i32,
        pub z: i32,
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
