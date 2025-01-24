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
