use beryllium::*;
use std::ffi::CString;

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
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap(); // Added this line
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    #[cfg(target_os = "linux")]
    {
        sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
    }
    let title: &str = "window";
    let win_args = video::CreateWinArgs {
        title: title,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };
    let window = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");
    gl::load_with(|s| {
        let c_str = CString::new(s).unwrap();
        unsafe { window.get_proc_address(c_str.as_ptr() as *const u8) as *const _ }
    });

    let (vao, shader_program) = unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(vertex_shader);
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(fragment_shader);
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        (vao, shader_program)
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
}
