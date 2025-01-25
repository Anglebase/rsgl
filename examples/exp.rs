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
    println!("GL initialized");

    while !window.should_close() {
        gl.render();
        window.swap_buffers();
        glfw.poll_events();
    }
    println!("GL released");
}

use rsgl::*;

struct GL<'a> {
    vao: VertexArray,
    _vbo: Buffer<'a, ArrayBuffer, f32>,
    program: Program,
}

impl<'a> GL<'a> {
    fn new() -> Self {
        let mut vao = VertexArray::new();
        let mut vbo = Buffer::new();

        vao.context(|vao| {
            vbo.context(|vbo| {
                vbo.buffer_data(&VTS, (Freq::Static, Mode::Draw));
            });
            vao.bind_vertex_attrib_pointer::<f32>(0, false, 3, 0);
        });

        let mut vs = Shader::new::<VertexShader>();
        vs.source(VS);
        vs.compile().unwrap();
        let mut fs = Shader::new::<FragmentShader>();
        fs.source(FS);
        fs.compile().unwrap();

        let mut program = Program::new();
        program.attach(&vs);
        program.attach(&fs);
        program.link().unwrap();

        Self {
            vao,
            _vbo: vbo,
            program,
        }
    }

    unsafe fn unsafe_render(&mut self) {
        gl::ClearColor(0.2, 0.4, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        use rsgl::Context;
        self.program.context(|_| {
            self.vao.context(|vao| {
                vao.draw_arrays(DrawMode::Triangles, 0, 3);
            });
        });
    }
}

impl<'a> GL<'a> {
    fn render(&mut self) {
        unsafe {
            self.unsafe_render();
        }
    }
}
