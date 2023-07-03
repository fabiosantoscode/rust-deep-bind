macro_rules! context {
    ($context_name:ident, $data_type:ty, $initial_value:expr) => {
        mod $context_name {
            use super::*;
            use std::cell::RefCell;

            thread_local! {
                pub(super) static T_LOCAL_CONTEXT: RefCell<$data_type> = RefCell::new($initial_value);
            }

            /// Get a clone of the current context value.
            pub(super) fn clone() -> $data_type {
                T_LOCAL_CONTEXT.with(|ctx| ctx.borrow().clone())
            }

            /// Provide a context value to a function and any functions that it calls.
            /// After the function returns, the context value is restored to its previous value.
            pub(super) fn replace_within<ContextfulFn: FnOnce() -> Ret, Ret>(
                data: $data_type,
                f: ContextfulFn,
            ) -> Ret {
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
    use std::rc::Rc;

    context!(ctx, u8, 42);

    #[test]
    fn test_number() {
        assert_eq!(ctx::clone(), 42_u8);
        assert_eq!(ctx::replace_within(43, || ctx::clone()), 43_u8);
        assert_eq!(ctx::clone(), 42_u8);
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Foo(u8);
    context!(foo, Foo, Foo(42));

    #[test]
    fn test_struct() {
        assert_eq!(foo::clone().0, 42_u8);
        assert_eq!(foo::replace_within(Foo(43), || foo::clone()).0, 43_u8);
        assert_eq!(foo::clone().0, 42_u8);
    }

    context!(foo_rc, Rc<Foo>, Rc::new(Foo(42)));

    #[test]
    fn test_rc() {
        assert_eq!(foo_rc::clone().0, 42_u8);
        assert_eq!(
            foo_rc::replace_within(Rc::new(Foo(43)), || foo_rc::clone()).0,
            43_u8
        );
        assert_eq!(foo_rc::clone().0, 42_u8);
    }
}
