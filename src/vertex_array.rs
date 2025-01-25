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

use ctxs::{Context, MutContext};
impl Context<Self> for VertexArray {
    fn context<R, F: FnOnce(&Self) -> R>(&self, local: F) -> R {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        local(self)
    }
}

impl MutContext<Self> for VertexArray {
    fn context<R, F: FnOnce(&mut Self) -> R>(&mut self, local: F) -> R {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        local(self)
    }
}

use crate::type_enum;

type_enum! {
    enum VertexAttribPointerType : GLenum {
        { f32 } : gl::FLOAT;
        { i32 } : gl::INT;
        { u32 } : gl::UNSIGNED_INT;
        { i8 } : gl::BYTE;
        { u8 } : gl::UNSIGNED_BYTE;
        { i16 } : gl::SHORT;
        { u16 } : gl::UNSIGNED_SHORT;
    }
}

impl VertexArray {
    pub fn bind_vertex_attrib_pointer<T: VertexAttribPointerType>(
        &self,
        index: u32,
        normalized: bool,
        stride: usize,
        offset: usize,
    ) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                std::mem::size_of::<T>() as i32,
                T::get_enum(),
                normalized as _,
                (stride * std::mem::size_of::<T>()) as _,
                offset as *const _,
            );
            gl::EnableVertexAttribArray(index);
        }
    }
}

pub enum DrawMode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl DrawMode {
    fn to_enum(self) -> GLenum {
        match self {
            DrawMode::Points => gl::POINTS,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::LineLoop => gl::LINE_LOOP,
            DrawMode::Lines => gl::LINES,
            DrawMode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            DrawMode::LinesAdjacency => gl::LINES_ADJACENCY,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            DrawMode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            DrawMode::Patches => gl::PATCHES,
        }
    }
}

impl VertexArray {
    pub fn draw_arrays(&self, mode: DrawMode, first: u32, count: u32) {
        let mode = mode.to_enum();
        unsafe {
            gl::DrawArrays(mode, first as _, count as _);
        }
    }
}

type_enum! {
    enum DrawElementsType : GLenum {
        { u8 } : gl::UNSIGNED_BYTE;
        { u16 } : gl::UNSIGNED_SHORT;
        { u32 } : gl::UNSIGNED_INT;
    }
}

impl VertexArray {
    pub fn draw_elements<T: DrawElementsType>(&self, mode: DrawMode, count: u32) {
        let mode = mode.to_enum();
        unsafe {
            use std::ptr::null;
            gl::DrawElements(mode, count as _, T::get_enum(), null());
        }
    }
}
