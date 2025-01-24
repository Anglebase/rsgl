//! 上下文管理器
//!
//! 上下文管理通常是基于栈变量的生命周期特性以及它的析构函数来实现的，\
//! 但这种方式通常算作隐式，并不直观，因此提供此特性以便于显式地体现上\
//! 下文管理的语义

/// 用于实现变量在不可变上下文中的上下文管理
///
/// # 示例
///
/// ```rust
/// use rsgl::Context;
///
/// struct MyContext {
///     value: i32,
/// }
///
/// impl Context<Self> for MyContext {
///     fn context<R, F: FnOnce(&Self) -> R>(&self, local: F) -> R {
///         local(self)
///     }
/// }
///
/// let mut ctx = MyContext { value: 10 };
/// let result = ctx.context(|ctx: &MyContext| {
///     ctx.value
/// });
/// assert_eq!(result, 10);
/// ```
pub trait Context<T> {
    /// 上下文环境
    ///
    /// # 参数
    ///
    /// - `local`: 一个闭包，它接受一个不可变的引用 `&T`
    ///
    /// # 返回值
    ///
    /// 应是闭包的返回值
    fn context<R, F: FnOnce(&T) -> R>(&self, local: F) -> R;
}

/// 用于实现变量在不可变上下文中具有内部可变性的上下文管理
///
/// 它通常用于 `Mutex`、`RefCell` 等含有内部可变性的类型，\
/// 此 crate 提供了对 `Mutex` 和 `RefCell` 的此 trait 的实现
///
/// # 示例
///
/// ```rust
/// use rsgl::ContextMut;
/// use std::sync::Mutex;
///
/// struct MyMutex<T>(Mutex<T>);  // 含有内部可变性的类型
///
/// impl<T> ContextMut<T> for MyMutex<T> {
///     fn context_mut<R, F: FnOnce(&mut T) -> R>(&self, local: F) -> R {
///         let mut lock = self.0.lock().unwrap();
///         local(&mut *lock)
///     }
/// }
///
/// let lock = MyMutex(Mutex::new(10));
/// let result = lock.context_mut(|value| {
///     assert_eq!(*value, 10);
///     *value = 20;
///     30
/// });
/// assert_eq!(lock.context_mut(|value| *value), 20);
/// ```
pub trait ContextMut<T> {
    /// 上下文环境
    ///
    /// # 参数
    ///
    /// - `local`: 一个闭包，它接受一个可变的引用 `&mut T`
    ///
    /// # 返回值
    ///
    /// 应是闭包的返回值
    fn context_mut<R, F: FnOnce(&mut T) -> R>(&self, local: F) -> R;
}

/// 用于实现变量在可变上下文中的上下文管理
///
/// 它用于没有内部可变性的类型的按需上下文管理，它与 `Context` 类似，\
/// 但它是可变的
///
/// # 示例
///
/// ```rust
/// use rsgl::MutContext;
///
/// struct MyStruct {
///     value: i32,
/// }
///
/// impl MutContext<Self> for MyStruct {
///     fn mut_context<R, F: FnOnce(&mut Self) -> R>(&mut self, local: F) -> R {
///         local(self)
///     }
/// }
///
/// let mut ctx = MyStruct { value: 10 };
/// ctx.mut_context(|ctx| {
///     assert_eq!(ctx.value, 10);
///     ctx.value = 20;
/// });
/// assert_eq!(ctx.value, 20);
/// ```
pub trait MutContext<T> {
    /// 上下文环境
    ///
    /// # 参数
    ///
    /// - `local`: 一个闭包，它接受一个可变的引用 `&mut T`
    ///
    /// # 返回值
    ///
    /// 应是闭包的返回值
    fn mut_context<R, F: FnOnce(&mut T) -> R>(&mut self, local: F) -> R;
}

use std::sync::Mutex;
impl<T> ContextMut<T> for Mutex<T> {
    /// 显式的 `Mutex` 上下文
    /// 
    /// # Panics
    /// 
    /// 如果 lock 时返回了错误，则会 panic
    fn context_mut<R, F: FnOnce(&mut T) -> R>(&self, local: F) -> R {
        let mut lock = self.lock().unwrap();
        local(&mut *lock)
    }
}

use std::cell::RefCell;
impl<T> ContextMut<T> for RefCell<T> {
    /// 显式的 `RefCell` 上下文
    fn context_mut<R, F: FnOnce(&mut T) -> R>(&self, local: F) -> R {
        let mut value = self.borrow_mut();
        local(&mut *value)
    }
}
