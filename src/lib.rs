#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! contextual {
    {$($visibility:vis $interface_name:ident($context_name:ident) : $data_type:ty = $initial_value:expr;)+} =>{
        $(
            contextual!($visibility $interface_name($context_name): $data_type = $initial_value);
        )+
    };

    ($visibility:vis $interface_name:ident($context_name:ident) : $data_type:ty = $initial_value:expr) => {
        thread_local! {
            $visibility static $context_name: std::cell::RefCell<$data_type>
                = std::cell::RefCell::new($initial_value);
        }

        $visibility struct $interface_name;

        #[allow(dead_code)]
        impl $interface_name {
            /// Get a clone of the current context value.
            pub fn clone() -> $data_type {
                $context_name.with(|ctx| ctx.borrow().clone())
            }

            /// Provide a context value to a function and any functions that it calls.
            /// After the function returns, the context value is restored to its previous value.
            pub fn replace_within<ContextZone: FnOnce() -> Ret, Ret>(
                data: $data_type,
                f: ContextZone,
            ) -> Ret {
                $context_name.with(|ctx| {
                    let old_data = ctx.replace(data);
                    let ret = f();
                    ctx.replace(old_data);
                    ret
                })
            }
        }
    };
}
