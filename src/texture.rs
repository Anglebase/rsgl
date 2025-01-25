use crate::buffer::TextureBuffer;
use gl::types::*;

use crate::type_enum;
type_enum! {
    enum TextureTarget : GLenum {
        Texture1D : gl::TEXTURE_1D;
        Texture2D : gl::TEXTURE_2D;
        Texture3D : gl::TEXTURE_3D;
        Texture1DArray : gl::TEXTURE_1D_ARRAY;
        Texture2DArray : gl::TEXTURE_2D_ARRAY;
        TextureRectangle : gl::TEXTURE_RECTANGLE;
        TextureCubeMap : gl::TEXTURE_CUBE_MAP;
        TextureCubeMapArray : gl::TEXTURE_CUBE_MAP_ARRAY;
        Texture2DMultisample : gl::TEXTURE_2D_MULTISAMPLE ;
        Texture2DMultisampleArray : gl::TEXTURE_2D_MULTISAMPLE_ARRAY;
        { TextureBuffer } : gl::TEXTURE_BUFFER;
    }
}

pub struct Texture<E>
where
    E: TextureTarget,
{
    _enum: E,
    id: GLuint,
}

impl<E> Texture<E>
where
    E: TextureTarget,
{
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Texture {
            _enum: E::default(),
            id,
        }
    }
}

impl<E> Drop for Texture<E>
where
    E: TextureTarget,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl<E> Texture<E>
where
    E: TextureTarget,
{
    pub fn bind(&self, active: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + active);
            gl::BindTexture(E::get_enum(), self.id);
        }
    }

    pub fn generate_mipmap(&self) -> Result<(), String> {
        let target = E::get_enum();
        match target {
            gl::TEXTURE_1D
            | gl::TEXTURE_2D
            | gl::TEXTURE_3D
            | gl::TEXTURE_1D_ARRAY
            | gl::TEXTURE_2D_ARRAY
            | gl::TEXTURE_CUBE_MAP
            | gl::TEXTURE_CUBE_MAP_ARRAY => {}
            _ => {
                return Err(format!(
                    "Invalid operation for texture type: {}",
                    std::any::type_name::<Self>()
                ));
            }
        };
        unsafe {
            gl::GenerateMipmap(target);
        }
        Ok(())
    }
}

use ctxs::Context;
impl<E> Context<Self> for Texture<E>
where
    E: TextureTarget,
{
    fn context<R, F: FnOnce(&Self) -> R>(&self, local: F) -> R {
        self.bind(0);
        local(self)
    }
}
