use gl::types::*;

pub struct Shader {
    pub(crate) id: GLuint,
}

use crate::type_enum;
type_enum! {
    enum ShaderType : GLenum {
        ComputeShader : gl::COMPUTE_SHADER;
        VertexShader : gl::VERTEX_SHADER;
        TessControlShader : gl::TESS_CONTROL_SHADER;
        TessEvaluationShader : gl::TESS_EVALUATION_SHADER;
        GeometryShader : gl::GEOMETRY_SHADER;
        FragmentShader : gl::FRAGMENT_SHADER;
    }
}

impl Shader {
    pub fn new<T: ShaderType>() -> Self {
        let id = unsafe { gl::CreateShader(T::get_enum()) };
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

impl Shader {
    pub fn source(&mut self, source: &str) {
        use std::ffi::CString;
        use std::ptr::null;
        let code_str = CString::new(source).unwrap();
        unsafe {
            gl::ShaderSource(self.id, 1, &code_str.as_ptr(), null());
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.id);
        }
        let mut success = gl::FALSE as GLint;
        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as GLint {
                use std::ptr::null_mut;
                let mut len = 0;
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len(len as usize);
                gl::GetShaderInfoLog(
                    self.id,
                    len,
                    null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                let info_log_str = String::from_utf8(info_log).unwrap();
                return Err(info_log_str);
            }
        }
        Ok(())
    }
}
