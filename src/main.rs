
mod chunk_manager;
mod renderer;

fn main()
{
        let mut renderer: renderer::Renderer = renderer::Renderer::new();

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

        unsafe {
                // Cleanup Code would go here
        }
}
