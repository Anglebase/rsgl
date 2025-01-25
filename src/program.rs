use gl::types::*;

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new() -> Self {
        Self {
            id: unsafe { gl::CreateProgram() },
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

use crate::Shader;
impl Program {
    pub fn attach(&mut self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
    }

    pub fn link(&mut self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.id);
        }
        let mut success = gl::FALSE as GLint;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
            if success == gl::FALSE as GLint {
                let mut len = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len(len as usize);
                gl::GetProgramInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(String::from_utf8(info_log).unwrap());
            }
        }
        Ok(())
    }
}

use ctxs::Context;
impl Context<Self> for Program {
    fn context<R, F: FnOnce(&Self) -> R>(&self, local: F) -> R {
        unsafe {
            gl::UseProgram(self.id);
        }
        local(self)
    }
}
