
mod chunk_manager;
mod renderer;

use renderer::*;
use chunk_manager::*;

fn main()
{
        let mut chunk_manager: ChunkManager = ChunkManager::new();
        let renderer: Renderer = Renderer::new();

        chunk_manager.add_player(Vector3::new(0.0, 0.0, 0.0), "Player".to_string());

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

                renderer.clear(renderer::Color::RED);
                renderer.swap();
        }
}
