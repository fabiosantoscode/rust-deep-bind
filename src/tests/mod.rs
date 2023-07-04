mod other_file;

use crate::contextual;

//use super::*;
use std::rc::Rc;

contextual!(ctx: u8 = 42);

#[test]
fn test_number() {
    assert_eq!(ctx::clone(), 42_u8);
    assert_eq!(ctx::replace_within(43, || ctx::clone()), 43_u8);
    assert_eq!(ctx::clone(), 42_u8);
}

#[derive(Debug, PartialEq, Clone)]
struct Foo(u8);
contextual! {
    foo: Foo = Foo(42);
    foo_rc: Rc<Foo> = Rc::new(Foo(42));
}

#[test]
fn test_struct() {
    assert_eq!(foo::clone().0, 42_u8);
    assert_eq!(foo::replace_within(Foo(43), || foo::clone()).0, 43_u8);
    assert_eq!(foo::clone().0, 42_u8);
}

#[test]
fn test_rc() {
    assert_eq!(foo_rc::clone().0, 42_u8);
    assert_eq!(
        foo_rc::replace_within(Rc::new(Foo(43)), || foo_rc::clone()).0,
        43_u8
    );
    assert_eq!(foo_rc::clone().0, 42_u8);
}

/*
#[test]
fn test_other_file() {
    use other_file::{from_another_file, MyStruct};

    assert_eq!(from_another_file::clone().0, 42_u8);
    assert_eq!(
        from_another_file::replace_within(MyStruct(43), || { from_another_file::clone() }).0,
        43_u8
    );
    assert_eq!(from_another_file::clone().0, 42_u8);
}
*/
