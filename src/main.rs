use beryllium::*;
use std::ffi::CString;
use std::ptr;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;
  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3)
        .expect("Failed to set GL major version");
    sdl.set_gl_context_minor_version(3)
        .expect("Failed to set GL minor version");
    sdl.set_gl_profile(video::GlProfile::Core)
        .expect("Failed to set GL profile");

    #[cfg(target_os = "macos")]
    {
        sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .expect("Failed to set GL context flags");
    }

    let win_args = video::CreateWinArgs {
        title: "Window",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };
    let window = sdl
        .create_gl_window(win_args)
        .expect("Couldn't make a window");

    gl::load_with(|s| {
        let c_str = CString::new(s).unwrap();
        unsafe { window.get_proc_address(c_str.as_ptr() as *const u8) as *const _ }
    });

    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let (vao, vbo, shader_program) = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTICES.len() * std::mem::size_of::<Vertex>()) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vert_source = CString::new(VERT_SHADER).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vert_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut log_len);
            let mut log = vec![0u8; log_len as usize];
            gl::GetShaderInfoLog(
                vertex_shader,
                log_len,
                &mut log_len,
                log.as_mut_ptr().cast(),
            );
            panic!(
                "Vertex shader compilation failed: {}",
                String::from_utf8_lossy(&log)
            );
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let frag_source = CString::new(FRAG_SHADER).unwrap();
        gl::ShaderSource(fragment_shader, 1, &frag_source.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0;
            gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut log_len);
            let mut log = vec![0u8; log_len as usize];
            gl::GetShaderInfoLog(
                fragment_shader,
                log_len,
                &mut log_len,
                log.as_mut_ptr().cast(),
            );
            panic!(
                "Fragment shader compilation failed: {}",
                String::from_utf8_lossy(&log)
            );
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0;
            gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut log_len);
            let mut log = vec![0u8; log_len as usize];
            gl::GetProgramInfoLog(
                shader_program,
                log_len,
                &mut log_len,
                log.as_mut_ptr().cast(),
            );
            panic!("Program linking failed: {}", String::from_utf8_lossy(&log));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        (vao, vbo, shader_program)
    };

    'mainloop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'mainloop,
                _ => (),
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_window();
    }

    unsafe {
        gl::DeleteProgram(shader_program);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
