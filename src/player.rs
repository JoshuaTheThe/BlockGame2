
use crate::vector::*;

#[derive(Clone)]
pub struct Player
{
        pub pos: Vector3,
        pub rot: Vector3,
        pub vel: Vector3,
        pub name: Option<String>,
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
                        rot: Vector3::new(0.0, 0.0, 0.0),
                }
        }
}
