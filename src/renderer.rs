
pub use beryllium::*;
pub use std::ffi::CString;
pub use std::ptr;

use crate::vector::*;
use cgmath::Matrix;
use cgmath::One;

const WINDOW_TITLE: &str = "Block Game 2";
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_ALLOW_DPI: bool = true;
const WINDOW_BORDERLESS: bool = false;
const WINDOW_RESIZABLE: bool = false;

#[repr(C)]
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

#[derive(Clone)]
pub struct Mesh
{
        pub vertices: Vec<Vertex>,
        pub indices: Vec<u32>,
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
        ebo: gl::types::GLuint,
        view_loc: gl::types::GLint,
        proj_loc: gl::types::GLint,
        model_loc: gl::types::GLint,
}

impl Renderer
{
        pub fn set_2d_mode(&self)
        {
                unsafe
                {
                        let projection = cgmath::ortho(
                                0.0, WINDOW_WIDTH as f32, 
                                WINDOW_HEIGHT as f32, 0.0, 
                                -1.0, 1.0
                        );
                        let view = cgmath::Matrix4::one();
                        let model = cgmath::Matrix4::one();
                        gl::UniformMatrix4fv(self.proj_loc, 1, gl::FALSE, projection.as_ptr());
                        gl::UniformMatrix4fv(self.view_loc, 1, gl::FALSE, view.as_ptr());
                        gl::UniformMatrix4fv(self.model_loc, 1, gl::FALSE, model.as_ptr());
                }
        }
            
        pub fn set_3d_mode(&self, eye: Vector3, target: Vector3, up: Vector3)
        {
                self.set_view_projection(eye, target, up);
        }

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
                        gl::UseProgram(self.program);
                        let one = cgmath::Matrix4::new(
                                1.0, 0.0, 0.0, 0.0,
                                0.0, 1.0, 0.0, 0.0,
                                0.0, 0.0, 1.0, 0.0,
                                0.0, 0.0, 0.0, 1.0
                        );
                        gl::UniformMatrix4fv(self.model_loc, 1, gl::FALSE, one.as_ptr());
                        gl::UniformMatrix4fv(self.view_loc, 1, gl::FALSE, one.as_ptr());
                        gl::UniformMatrix4fv(self.proj_loc, 1, gl::FALSE, one.as_ptr());
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
                let ndc_x = (x / WINDOW_WIDTH as f32) * 2.0 - 1.0;
                let ndc_y = (y / WINDOW_HEIGHT as f32) * 2.0 - 1.0;
                let ndc_width = (width / WINDOW_WIDTH as f32) * 2.0;
                let ndc_height = (height / WINDOW_HEIGHT as f32) * 2.0;
            
                let vertices = [
                        Vertex::new(ndc_x, ndc_y, 0.0, color),
                        Vertex::new(ndc_x + ndc_width, ndc_y, 0.0, color),
                        Vertex::new(ndc_x, ndc_y + ndc_height, 0.0, color),
                        Vertex::new(ndc_x + ndc_width, ndc_y, 0.0, color),
                        Vertex::new(ndc_x + ndc_width, ndc_y + ndc_height, 0.0, color),
                        Vertex::new(ndc_x, ndc_y + ndc_height, 0.0, color),
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
                        uniform mat4 model;
                        uniform mat4 view;
                        uniform mat4 projection;
                        out vec4 vColor;
                        
                        // Constant matrix to convert from Z-up to Y-up
                        const mat4 zUpToYUp = mat4(
                            1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0, 0.0, 0.0,
                            0.0, 0.0, 1.0, 0.0,
                            0.0, 0.0, 0.0, 1.0
                        );
                        
                        void main()
                        {
                            gl_Position = projection * view * model * zUpToYUp * vec4(aPos, 1.0);
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
        
        fn setup_buffers() -> (gl::types::GLuint, gl::types::GLuint, gl::types::GLuint)
        {
                let mut vao = 0;
                let mut vbo = 0;
                let mut ebo = 0;
                
                unsafe
                {
                    gl::GenVertexArrays(1, &mut vao);
                    gl::GenBuffers(1, &mut vbo);
                    gl::GenBuffers(1, &mut ebo);
                    gl::BindVertexArray(vao);
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
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
                
                (vao, vbo, ebo)
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
                let (vao, vbo, ebo) = Self::setup_buffers();

                unsafe
                {
                        gl::Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
                        gl::UseProgram(program);
                        gl::Enable(gl::DEPTH_TEST);
                }

                let view_loc = unsafe
                {
                        gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr())
                };
                if view_loc == -1
                {
                        panic!("Uniform 'view' not found in shader");
                }

                let proj_loc = unsafe
                {
                        gl::GetUniformLocation(program, CString::new("projection").unwrap().as_ptr())
                };
                if proj_loc == -1
                {
                        panic!("Uniform 'projection' not found in shader");
                }

                let model_loc = unsafe
                {
                        gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr())
                };
                if model_loc == -1
                {
                        panic!("Uniform 'model' not found in shader");
                }

                Renderer
                {
                        sdl,
                        window,
                        program,
                        vao,
                        vbo,
                        ebo,
                        view_loc,
                        proj_loc,
                        model_loc,
                }
        }

        pub fn set_view_projection(&self, eye: Vector3, target: Vector3, up: Vector3)
        {
                use cgmath::{Matrix4, Vector3 as CGVector3, Point3, Deg};
                let eye = Point3::new(eye.x, eye.y, eye.z);
                let target = Point3::new(target.x, target.y, target.z);
                let up = CGVector3::new(up.x, up.y, up.z);
                let view = Matrix4::look_at(eye, target, up);
                let aspect = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
                let fov = Deg(70.0);
                let near = 0.1;
                let far = 1000.0;
                let projection = cgmath::perspective(fov, aspect, near, far);
            
                unsafe
                {
                        gl::UniformMatrix4fv(self.view_loc, 1, gl::FALSE, view.as_ptr());
                        gl::UniformMatrix4fv(self.proj_loc, 1, gl::FALSE, projection.as_ptr());
                }
        }

        pub fn draw_mesh(&self, mesh: &Mesh, pos: Vector3)
        {
                unsafe
                {
                        gl::UseProgram(self.program);
                        let model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(pos.x, pos.y, pos.z));
                        gl::UniformMatrix4fv(self.model_loc, 1, gl::FALSE, model.as_ptr());
                        gl::BindVertexArray(self.vao);
                        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                        gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                                mesh.vertices.as_ptr() as *const _,
                                gl::STATIC_DRAW,
                        );
                
                        if !mesh.indices.is_empty()
                        {
                                gl::BufferData(
                                        gl::ELEMENT_ARRAY_BUFFER,
                                        (mesh.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                                        mesh.indices.as_ptr() as *const _,
                                        gl::STATIC_DRAW,
                                );
                    
                                gl::DrawElements(
                                        gl::TRIANGLES,
                                        mesh.indices.len() as i32,
                                        gl::UNSIGNED_INT,
                                        std::ptr::null()
                                );
                        }
                        else
                        {
                                gl::DrawArrays(gl::TRIANGLES, 0, mesh.vertices.len() as i32);
                        }
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
                        gl::DeleteBuffers(1, &self.ebo);
                }
        }
}

fn check_gl_error()
{
        unsafe
        {
                let err = gl::GetError();
                if err != gl::NO_ERROR
                {
                        panic!("OpenGL error: {}", err);
                }
        }
}
