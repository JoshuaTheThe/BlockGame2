
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

        'mainloop: loop
        {
                while let Some(event) = renderer.get_sdl().poll_events()
                {
                        match event
                        {
                                (renderer::events::Event::Quit, _) => break 'mainloop,
                                _ => (),
                        }
                }

                chunk_manager.load_chunks();
                chunk_manager.remove_chunks();

                if chunk_manager.needs_mesh_update()
                {
                        chunk_manager.generate_meshes();
                }

                renderer.clear(renderer::Color::BLACK);
                for mesh in chunk_manager.get_meshes()
                {
                        renderer.draw_mesh(mesh, Vector3::new(0.00,0.00,0.00));
                }
                renderer.swap();
        }
}
