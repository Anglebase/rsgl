use gl::types::*;

pub struct Shader {
    id: GLuint,
}

pub trait ShaderType {
    fn get_type() -> GLenum;
}

pub mod shader_type {
    use super::*;
    pub struct VertexShader;
    impl ShaderType for VertexShader {
        fn get_type() -> GLenum {
            gl::VERTEX_SHADER
        }
    }
    pub struct FragmentShader;
    impl ShaderType for FragmentShader {
        fn get_type() -> GLenum {
            gl::FRAGMENT_SHADER
        }
    }
}

impl Shader {
    pub fn new<T: ShaderType>() -> Self {
        let id = unsafe { gl::CreateShader(T::get_type()) };
        Self { id }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
