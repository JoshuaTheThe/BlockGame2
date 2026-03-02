
mod terrain;
mod chunk;
mod player;
mod vector;
mod chunk_manager;
mod renderer;

use renderer::*;
use chunk_manager::*;
use vector::*;

const MOUSE_SENSITIVITY: f32 = 1.0;
const MAX_PITCH: f32 = 0.0;
const MIN_PITCH: f32 = -180.0;
const MOVE_SPEED: f32 = 1.0;

fn main()
{
        let mut chunk_manager: ChunkManager = ChunkManager::new();
        let renderer: Renderer = Renderer::new();
        let player_name = "Player".to_string();

        chunk_manager.add_player(Vector3::new(0.0, 0.0, 10.0), player_name.clone());
        chunk_manager.load_chunks();
        chunk_manager.generate_meshes();

        'mainloop: loop
        {
                if let Some(player) = chunk_manager.get_player(&player_name) {
                        renderer.set_view_projection_from_rot(
                                player.pos,
                                player.rot,
                        );
                }
                
                while let Some(event) = renderer.get_sdl().poll_events()
                {
                        match event
                        {
                                (renderer::events::Event::WindowCloseRequest { .. }, _) => break 'mainloop,
                                (renderer::events::Event::Quit, _) => break 'mainloop,
                                (renderer::events::Event::Key { keycode, pressed, .. }, _) => {
                                        if pressed
                                        {
                                                match keycode {
                                                        beryllium::events::SDLK_ESCAPE => break 'mainloop,
                                                        beryllium::events::SDLK_w => {
                                                                if let Some(player) = chunk_manager.get_player_mut(&player_name)
                                                                {
                                                                        let yaw_rad = player.rot.z.to_radians();
                                                                        player.pos.x += yaw_rad.sin() * MOVE_SPEED;
                                                                        player.pos.y += yaw_rad.cos() * MOVE_SPEED;
                                                                }
                                                        },
                                                        beryllium::events::SDLK_s => {
                                                                if let Some(player) = chunk_manager.get_player_mut(&player_name)
                                                                {
                                                                        let yaw_rad = player.rot.z.to_radians();
                                                                        player.pos.x -= yaw_rad.sin() * MOVE_SPEED;
                                                                        player.pos.y -= yaw_rad.cos() * MOVE_SPEED;
                                                                }
                                                        },
                                                        beryllium::events::SDLK_a => {
                                                                if let Some(player) = chunk_manager.get_player_mut(&player_name)
                                                                {
                                                                        let yaw_rad = player.rot.z.to_radians();
                                                                        player.pos.x -= yaw_rad.cos() * MOVE_SPEED;
                                                                        player.pos.y += yaw_rad.sin() * MOVE_SPEED;
                                                                }
                                                        },
                                                        beryllium::events::SDLK_d => {
                                                                if let Some(player) = chunk_manager.get_player_mut(&player_name)
                                                                {
                                                                        let yaw_rad = player.rot.z.to_radians();
                                                                        player.pos.x += yaw_rad.cos() * MOVE_SPEED;
                                                                        player.pos.y -= yaw_rad.sin() * MOVE_SPEED;
                                                                }
                                                        },
                                                        beryllium::events::SDLK_q => {
                                                                if let Some(player) = chunk_manager.get_player_mut(&player_name)
                                                                {
                                                                        player.pos.z -= MOVE_SPEED;
                                                                }
                                                        }
                                                        beryllium::events::SDLK_e => {
                                                                if let Some(player) = chunk_manager.get_player_mut(&player_name)
                                                                {
                                                                        player.pos.z += MOVE_SPEED;
                                                                }
                                                        }
                                                        _ => {},
                                                }
                                        }
                                },
                                (renderer::events::Event::MouseMotion { x_delta, y_delta, .. }, _) => {
                                        if let Some(player) = chunk_manager.get_player_mut(&player_name) {
                                                player.rot.y += y_delta as f32 * MOUSE_SENSITIVITY;
                                                player.rot.z += x_delta as f32 * MOUSE_SENSITIVITY;
                                                if player.rot.y > MAX_PITCH
                                                {
                                                        player.rot.y = MAX_PITCH - 0.1;
                                                }
                                                if player.rot.y < MIN_PITCH
                                                {
                                                        player.rot.y = MIN_PITCH + 0.1;
                                                }
                                        }
                                },
                                _ => (),
                        }
                }

                renderer.clear(Color::new(0.1, 0.1, 0.5, 1.0));
                chunk_manager.load_chunks();
                chunk_manager.remove_chunks();

                for (i, mesh) in chunk_manager.get_meshes().iter().enumerate()
                {
                        if let Some(chunk) = chunk_manager.get_chunk(i)
                        {
                                let world_x = (chunk.xy.x * chunk::CHUNK_SIZE as i32) as f32;
                                let world_y = (chunk.xy.y * chunk::CHUNK_SIZE as i32) as f32;
                                renderer.draw_mesh(mesh, Vector3::new(world_x, world_y, 0.0));
                        }
                }

                renderer.set_2d_mode();
                renderer.draw_rect(1.0, 1.0, 128.0, 128.0, Color::WHITE);
                renderer.swap();
        }
}
