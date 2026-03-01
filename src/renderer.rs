
pub use beryllium::*;
pub use std::ffi::CString;
pub use std::ptr;

const WINDOW_TITLE: &str = "Block Game 2";
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_ALLOW_DPI: bool = true;
const WINDOW_BORDERLESS: bool = false;
const WINDOW_RESIZABLE: bool = false;

#[derive(Clone, Copy)]
pub struct Color
{
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
}

impl Color 
{
        pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self
        {
            Self { r, g, b, a }
        }
        
        pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
        pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
        pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
        pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
        pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex
{
        pub position: [f32; 3],
        pub color: Color,
}

impl Vertex
{
        pub fn new(x: f32, y: f32, z: f32, color: Color) -> Self
        {
                Self
                {
                        position: [x, y, z],
                        color: color,
                }
        }
}
    

pub struct Renderer
{
        sdl: Sdl,
        window: video::GlWindow,
        program: gl::types::GLuint,
        vao: gl::types::GLuint,
        vbo: gl::types::GLuint,
}

impl Renderer
{
        pub fn get_sdl(&self) -> &Sdl
        {
                &self.sdl
        }

        pub fn get_window(&self) -> &video::GlWindow
        {
                &self.window
        }

        pub fn clear(&self, color: Color)
        {
                unsafe
                {
                    gl::ClearColor(color.r, color.g, color.b, color.a);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
        }

        pub fn draw_tris(&self, vertices: &[Vertex])
        {
                unsafe
                {
                        gl::BindVertexArray(self.vao);
                        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                        gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                                vertices.as_ptr() as *const _,
                                gl::DYNAMIC_DRAW,
                        );
            
                        gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
                }
        }

        pub fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32, color: Color)
        {
                let vertices = [
                    Vertex::new(x, y, 0.0, color),
                    Vertex::new(x + width, y, 0.0, color),
                    Vertex::new(x, y + height, 0.0, color),
                    Vertex::new(x + width, y, 0.0, color),
                    Vertex::new(x + width, y + height, 0.0, color),
                    Vertex::new(x, y + height, 0.0, color),
                ];
                self.draw_tris(&vertices);
        }

        pub fn swap(&self)
        {
                self.window.swap_window();
        }

        fn create_shader_program() -> gl::types::GLuint
        {
                let vertex_shader = Self::compile_shader(
                        gl::VERTEX_SHADER,
                        r#"
                        #version 330 core
                        layout (location = 0) in vec3 aPos;
                        layout (location = 1) in vec4 aColor;
                        out vec4 vColor;
                        void main()
                        {
                                gl_Position = vec4(aPos, 1.0);
                                vColor = aColor;
                        }"#,
                );
                
                let fragment_shader = Self::compile_shader(
                        gl::FRAGMENT_SHADER,
                        r#"
                        #version 330 core
                        in vec4 vColor;
                        out vec4 FragColor;
                        void main()
                        {
                                FragColor = vColor;
                        }"#,
                );
                
                unsafe
                {
                        let program = gl::CreateProgram();
                        gl::AttachShader(program, vertex_shader);
                        gl::AttachShader(program, fragment_shader);
                        gl::LinkProgram(program);
                    
                        let mut success = gl::FALSE as gl::types::GLint;
                        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
                        if success != gl::TRUE as gl::types::GLint
                        {
                                panic!("Shader linking failed");
                        }
                        gl::DeleteShader(vertex_shader);
                        gl::DeleteShader(fragment_shader);
                        program
                }
        }
        
        fn compile_shader(shader_type: gl::types::GLenum, source: &str) -> gl::types::GLuint
        {
                unsafe
                {
                        let shader = gl::CreateShader(shader_type);
                        let c_str = std::ffi::CString::new(source).unwrap();
                        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
                        gl::CompileShader(shader);
                        let mut success = gl::FALSE as gl::types::GLint;
                        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                        if success != gl::TRUE as gl::types::GLint
                        {
                                panic!("Shader compilation failed");
                        }
                        shader
                }
        }
        
        fn setup_buffers() -> (gl::types::GLuint, gl::types::GLuint)
        {
                let mut vao = 0;
                let mut vbo = 0;
                
                unsafe
                {
                    gl::GenVertexArrays(1, &mut vao);
                    gl::GenBuffers(1, &mut vbo);
                    gl::BindVertexArray(vao);
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    gl::VertexAttribPointer(
                        0, 3, gl::FLOAT, gl::FALSE,
                        std::mem::size_of::<Vertex>() as i32,
                        std::ptr::null()
                    );
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(
                        1, 4, gl::FLOAT, gl::FALSE,
                        std::mem::size_of::<Vertex>() as i32,
                        (3 * std::mem::size_of::<f32>()) as *const _
                    );
                    gl::EnableVertexAttribArray(1);
                }
                
                (vao, vbo)
        }

        pub fn new() -> Self
        {
                let sdl = Sdl::init(init::InitFlags::EVERYTHING);
                sdl.set_gl_context_major_version(3).expect("Failed to set GL major version");
                sdl.set_gl_context_minor_version(3).expect("Failed to set GL minor version");
                sdl.set_gl_profile(video::GlProfile::Core).expect("Failed to set GL profile");
                #[cfg(target_os = "macos")]
                {
                        sdl.gl_set_context_flags(GlContextFlags::FORWARD_COMPATIBLE).expect("Failed to set GL context flags");
                }
                
                let win_args = video::CreateWinArgs {
                        title: WINDOW_TITLE,
                        width: WINDOW_WIDTH,
                        height: WINDOW_HEIGHT,
                        allow_high_dpi: WINDOW_ALLOW_DPI,
                        borderless: WINDOW_BORDERLESS,
                        resizable: WINDOW_RESIZABLE,
                };

                let window = sdl.create_gl_window(win_args).expect("Couldn't create window");
                gl::load_with(|s| {
                        let c_str = CString::new(s).unwrap();
                        unsafe {
                                window.get_proc_address(c_str.as_ptr() as *const u8) as *const _
                        }
                });

                let program = Self::create_shader_program();
                let (vao, vbo) = Self::setup_buffers();

                unsafe
                {
                        gl::UseProgram(program);
                        gl::Enable(gl::DEPTH_TEST);
                }
                Renderer
                {
                        sdl,
                        window,
                        program,
                        vao,
                        vbo,
                }
        }
}

impl Drop for Renderer
{
        fn drop(&mut self)
        {
                unsafe
                {
                        gl::DeleteProgram(self.program);
                        gl::DeleteVertexArrays(1, &self.vao);
                        gl::DeleteBuffers(1, &self.vbo);
                }
        }
}
