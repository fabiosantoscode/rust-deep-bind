mod other_file;

use crate::contextual;

//use super::*;
use std::rc::Rc;

contextual!(Ctx(CTX): u8 = 42);

#[test]
fn test_number() {
    assert_eq!(Ctx::clone(), 42_u8);
    assert_eq!(Ctx::replace_within(43, || Ctx::clone()), 43_u8);
    assert_eq!(Ctx::clone(), 42_u8);
}

#[derive(Debug, PartialEq, Clone)]
struct Foo(u8);
contextual! {
    FooStruct(FOO): Foo = Foo(42);
    FooRc(FOO_RC): Rc<Foo> = Rc::new(Foo(42));
}

#[test]
fn test_struct() {
    assert_eq!(FooStruct::clone().0, 42_u8);
    assert_eq!(
        FooStruct::replace_within(Foo(43), || FooStruct::clone()).0,
        43_u8
    );
    assert_eq!(FooStruct::clone().0, 42_u8);
}

#[test]
fn test_deep() {
    assert_eq!(
        vec![
            vec![Ctx::clone()],
            Ctx::replace_within(43, || vec![
                Ctx::clone(),
                Ctx::replace_within(44, Ctx::clone),
                Ctx::clone()
            ]),
            vec![Ctx::clone()]
        ],
        vec![vec![42_u8], vec![43_u8, 44_u8, 43_u8], vec![42_u8]]
    )
}

#[test]
fn test_rc() {
    assert_eq!(FooRc::clone().0, 42_u8);
    assert_eq!(
        FooRc::replace_within(Rc::new(Foo(43)), || FooRc::clone()).0,
        43_u8
    );
    assert_eq!(FooRc::clone().0, 42_u8);
}

#[test]
fn test_other_file() {
    use other_file::{FromAnotherFile, MyStruct};

    assert_eq!(FromAnotherFile::clone().0, 42_u8);
    assert_eq!(
        FromAnotherFile::replace_within(MyStruct(43), || { FromAnotherFile::clone() }).0,
        43_u8
    );
    assert_eq!(FromAnotherFile::clone().0, 42_u8);
}
