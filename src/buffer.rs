use crate::type_enum;
use gl::types::*;

type_enum! {
    enum BufferType : GLenum {
        ArrayBuffer : gl::ARRAY_BUFFER;
        AtomicCounterBuffer : gl::ATOMIC_COUNTER_BUFFER;
        CopyReadBuffer : gl::COPY_READ_BUFFER;
        CopyWriteBuffer : gl::COPY_WRITE_BUFFER;
        DispatchIndirectBuffer : gl::DISPATCH_INDIRECT_BUFFER;
        DrawIndirectBuffer : gl::DRAW_INDIRECT_BUFFER;
        ElementArrayBuffer : gl::ELEMENT_ARRAY_BUFFER;
        PixelPackBuffer : gl::PIXEL_PACK_BUFFER;
        PixelUnpackBuffer : gl::PIXEL_UNPACK_BUFFER;
        QueryBuffer : gl::QUERY_BUFFER;
        ShaderStorageBuffer : gl::SHADER_STORAGE_BUFFER;
        TextureBuffer : gl::TEXTURE_BUFFER;
        TransformFeedbackBuffer : gl::TRANSFORM_FEEDBACK_BUFFER;
        UniformBuffer : gl::UNIFORM_BUFFER;
    }
}

pub struct Buffer<'a, E, T>
where
    E: BufferType,
{
    _enum: E,
    _data: Option<&'a [T]>,
    id: GLuint,
}

impl<'a, E, T> Buffer<'a, E, T>
where
    E: BufferType,
{
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            id,
            _enum: E::default(),
            _data: None,
        }
    }
}

impl<'a, E, T> Drop for Buffer<'a, E, T>
where
    E: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

use ctxs::{Context, MutContext};

impl<'a, E, T> Context<Self> for Buffer<'a, E, T>
where
    E: BufferType,
{
    fn context<R, F: FnOnce(&Self) -> R>(&self, local: F) -> R {
        unsafe {
            gl::BindBuffer(E::get_enum(), self.id);
        }
        local(self)
    }
}

impl<'a, E, T> MutContext<Self> for Buffer<'a, E, T>
where
    E: BufferType,
{
    fn context<R, F: FnOnce(&mut Self) -> R>(&mut self, local: F) -> R {
        unsafe {
            gl::BindBuffer(E::get_enum(), self.id);
        }
        local(self)
    }
}

pub enum Freq {
    Stream,
    Static,
    Dynamic,
}

pub enum Mode {
    Draw,
    Read,
    Copy,
}

impl<'a, E: BufferType, T: Sized> Buffer<'a, E, T> {
    pub fn buffer_data(&'a self, data: &'a [T], usage: (Freq, Mode)) {
        let usage = match usage {
            (Freq::Stream, Mode::Draw) => gl::STREAM_DRAW,
            (Freq::Stream, Mode::Read) => gl::STREAM_READ,
            (Freq::Stream, Mode::Copy) => gl::STREAM_COPY,
            (Freq::Static, Mode::Draw) => gl::STATIC_DRAW,
            (Freq::Static, Mode::Read) => gl::STATIC_READ,
            (Freq::Static, Mode::Copy) => gl::STATIC_COPY,
            (Freq::Dynamic, Mode::Draw) => gl::DYNAMIC_DRAW,
            (Freq::Dynamic, Mode::Read) => gl::DYNAMIC_READ,
            (Freq::Dynamic, Mode::Copy) => gl::DYNAMIC_COPY,
        };
        let size = data.len() * std::mem::size_of::<T>();
        unsafe {
            gl::BufferData(
                E::get_enum(),
                size as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage,
            );
        }
    }
}
