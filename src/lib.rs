#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! contextual {
    {$($visibility:vis $context_name:ident : $data_type:ty = $initial_value:expr;)+} =>{
        $(
            contextual!($visibility $context_name: $data_type = $initial_value);
        )+
    };

    ($visibility:vis $context_name:ident : $data_type:ty = $initial_value:expr) => {
        $visibility mod $context_name {
            #![allow(unused_imports)]
            #![allow(dead_code)]

            use super::*;
            use std::cell::RefCell;

            thread_local! {
                pub(super) static THREAD_LOCAL_CTX_HOLDER: RefCell<$data_type>
                    = RefCell::new($initial_value);
            }

            /// Get a clone of the current context_inner value.
            pub(super) fn clone() -> $data_type {
                THREAD_LOCAL_CTX_HOLDER.with(|ctx| ctx.borrow().clone())
            }

            /// Provide a context value to a function and any functions that it calls.
            /// After the function returns, the context value is restored to its previous value.
            pub(super) fn replace_within<ContextZone: FnOnce() -> Ret, Ret>(
                data: $data_type,
                f: ContextZone,
            ) -> Ret {
                THREAD_LOCAL_CTX_HOLDER.with(|ctx| {
                    let old_data = ctx.replace(data);
                    let ret = f();
                    ctx.replace(old_data);
                    ret
                })
            }
        }
    };
}
