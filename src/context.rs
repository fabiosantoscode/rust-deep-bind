use std::cell::RefCell;
use std::ops::Deref;

pub struct Context<T> {
    data: T,
}

impl<T> Deref for Context<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> Context<T> {
    fn new(data: T) -> Self {
        Context { data }
    }
}

macro_rules! context {
    ($name:ident, $d:ty, $val:expr) => {
        mod $name {
            use super::*;

            thread_local! {
                pub static T_LOCAL_CONTEXT: RefCell<$d> = RefCell::new($val);
            }

            pub fn get<'a>() -> $d {
                T_LOCAL_CONTEXT.with(|ctx| ctx.borrow().clone())
            }

            pub fn with<F: FnOnce() -> R, R>(data: $d, f: F) -> R {
                T_LOCAL_CONTEXT.with(|ctx| {
                    let old_data = ctx.replace(data);
                    let ret = f();
                    ctx.replace(old_data);
                    ret
                })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    context!(CTX, u8, 42);

    #[test]
    fn test_context() {
        assert_eq!(CTX::get(), 42_u8);

        let inner = CTX::with(43_u8, || CTX::get());

        assert_eq!(inner, 43_u8);
        assert_eq!(CTX::get(), 42_u8);
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Foo {
        x: u8,
    }
    context!(FOO, Foo, Foo { x: 42 });

    #[test]
    fn test_context_struct() {
        assert_eq!(FOO::get().x, 42);

        let inner = FOO::with(Foo { x: 43 }, || FOO::get());

        assert_eq!(inner.x, 43);
        assert_eq!(FOO::get().x, 42);
    }
}
