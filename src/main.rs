
mod chunk_manager;
mod renderer;

use renderer::*;
use chunk_manager::*;

fn main()
{
        let mut chunk_manager: ChunkManager = ChunkManager::new();
        let renderer: Renderer = Renderer::new();

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
                renderer.get_window().swap_window();
        }
}
