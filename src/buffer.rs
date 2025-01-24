use gl::types::*;

pub struct Buffer {
    id: GLuint,
}

impl Buffer {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub mod buffer_type {
    pub struct ArrayBuffer;
    pub struct ElementArrayBuffer;
}

use ctxs::Context;
impl Context<buffer_type::ArrayBuffer> for Buffer {
    fn context<R, F: FnOnce(&buffer_type::ArrayBuffer) -> R>(&self, local: F) -> R {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
        local(&buffer_type::ArrayBuffer)
    }
}

impl Context<buffer_type::ElementArrayBuffer> for Buffer {
    fn context<R, F: FnOnce(&buffer_type::ElementArrayBuffer) -> R>(&self, local: F) -> R {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
        local(&buffer_type::ElementArrayBuffer)
    }
}
