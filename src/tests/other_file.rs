use crate::contextual;

#[derive(Clone)]
pub(super) struct MyStruct(pub(super) u8);

contextual! {
    pub(super) from_another_file: MyStruct = MyStruct(42);
}
