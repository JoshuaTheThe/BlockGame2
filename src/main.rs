use beryllium::*;

fn main()
{
        let sdl = Sdl::init(init::InitFlags::EVERYTHING);
        sdl.set_gl_context_major_version(3).unwrap();
        sdl.set_gl_context_major_version(3).unwrap();
        sdl.set_gl_profile(video::GlProfile::Core).unwrap();
        #[cfg(target_os = "linux")]
        {
          sdl
            .set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
        }

        let title: &str = "window";
        let win_args = video::CreateWinArgs
        {
                title: title,
                width: 800,
                height: 600,
                allow_high_dpi: true,
                borderless: false,
                resizable: false,
        };
        
        let _win = sdl.create_gl_window(win_args).expect("couldn't make a window and context");
        'main_loop: loop
        {
                while let Some(event) = sdl.poll_events()
                {
                        match event
                        {
                                (events::Event::Quit, _) => break 'main_loop,
                                _ => (),
                        }
                }
                // now the events are clear
        
                // here's where we could change the world state and draw.
        }            
}
