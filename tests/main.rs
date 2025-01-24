const VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    void main()
    {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
    "#;
const FS: &str = r#"
    #version 330 core
    out vec4 FragColor;

    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
    "#;
const VTS: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
use gl::types::*;
use glfw::*;

fn main() {
    let mut glfw = glfw::init(fail_on_errors).unwrap();
    println!("GLFW initialized");

    let (mut window, _) = glfw
        .create_window(800, 600, "LearnOpenGL", WindowMode::Windowed)
        .unwrap();
    println!("Window created");

    window.make_current();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    println!("OpenGL loaded");

    let mut gl = GL::new();
    gl.init();
    println!("GL initialized");

    while !window.should_close() {
        gl.render();
        window.swap_buffers();
        glfw.poll_events();
    }
    gl.release();
    println!("GL released");
}

struct GL {
    vao: u32,
    vbo: u32,
    program: u32,
}

impl GL {
    fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            program: 0,
        }
    }

    unsafe fn unsafe_init(&mut self) {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VTS.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            VTS.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        self.vao = vao;
        self.vbo = vbo;
        println!("VAO and VBO initialized");

        let vs_id = gl::CreateShader(gl::VERTEX_SHADER);
        let fs_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        let vs_code = std::ffi::CString::new(VS).unwrap();
        let fs_code = std::ffi::CString::new(FS).unwrap();
        gl::ShaderSource(vs_id, 1, vs_code.as_ptr() as _, std::ptr::null());
        gl::ShaderSource(fs_id, 1, fs_code.as_ptr() as _, std::ptr::null());
        gl::CompileShader(vs_id);
        gl::CompileShader(fs_id);
        println!("Vertex and Fragment shaders compiled");

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs_id);
        gl::AttachShader(program, fs_id);
        gl::LinkProgram(program);
        self.program = program;
        println!("Shader program linked");

        gl::DeleteShader(vs_id);
        gl::DeleteShader(fs_id);
        println!("Unused shaders deleted");

        let pos_attrib = gl::GetAttribLocation(program, "aPos".as_ptr() as *const GLchar);
        gl::VertexAttribPointer(
            pos_attrib as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            0,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(pos_attrib as GLuint);
        println!("Vertex attribute pointer set and enabled");
    }

    unsafe fn unsafe_render(&mut self) {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        println!("Frame cleared");
    }

    unsafe fn unsafe_release(&mut self) {
        gl::DeleteProgram(self.program);
        gl::DeleteVertexArrays(1, &self.vao);
        gl::DeleteBuffers(1, &self.vbo);
        println!("Resources released");
    }
}

impl GL {
    fn init(&mut self) {
        unsafe {
            self.unsafe_init();
        }
    }

    fn render(&mut self) {
        unsafe {
            self.unsafe_render();
        }
    }

    fn release(&mut self) {
        unsafe {
            self.unsafe_release();
        }
    }
}
