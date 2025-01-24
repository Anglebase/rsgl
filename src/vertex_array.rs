use gl::types::*;

pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

use ctxs::Context;
impl Context<Self> for VertexArray {
    fn context<R, F: FnOnce(&Self) -> R>(&self, local: F) -> R {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        local(self)
    }
}
