
mod chunk;
mod player;
mod vector;
mod chunk_manager;
mod renderer;

use renderer::*;
use chunk_manager::*;
use vector::*;

fn main()
{
        let mut chunk_manager: ChunkManager = ChunkManager::new();
        let renderer: Renderer = Renderer::new();

        chunk_manager.add_player(Vector3::new(0.0, 0.0, 0.0), "Player".to_string());
        chunk_manager.load_chunks();
        chunk_manager.generate_meshes();

        'mainloop: loop
        {
                renderer.set_view_projection(
                        Vector3::new(0.0, 0.1, 100.0),
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(0.0, 0.0, 1.0)
                );
                while let Some(event) = renderer.get_sdl().poll_events()
                {
                        match event
                        {
                                (renderer::events::Event::Quit, _) => break 'mainloop,
                                _ => (),
                        }
                }

                renderer.clear(Color::new(0.1, 0.1, 0.5, 1.0));

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
