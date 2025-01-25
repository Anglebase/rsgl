mod buffer;
mod program;
mod shader;
mod vertex_array;

pub use buffer::*;
pub use program::*;
pub use shader::*;
pub use vertex_array::*;
pub use ctxs::*;

/// 定义元编程类型枚举值
#[macro_export]
macro_rules! type_enum {
    { enum $name:ident : $value_type:ty {
         $( $variant:ident : $value:expr ;)*
         $( { $variant_std:ident } : $value_std:expr ;)*
         $(,)?
    } } => {
        pub trait $name : Default {
            fn get_enum() -> $value_type;
        }
        $(
            #[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $variant;
            impl $name for $variant {
                fn get_enum() -> $value_type {
                    $value
                }
            }
            impl std::fmt::Debug for $variant {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "Enum {} : {}", stringify!($name), stringify!($variant))
                }
            }
        )*
        $(
            impl $name for $variant_std {
                fn get_enum() -> $value_type {
                    $value_std
                }
            }
        )*
    };
}