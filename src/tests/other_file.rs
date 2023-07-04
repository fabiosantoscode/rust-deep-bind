use crate::contextual;

#[derive(Clone)]
pub(super) struct MyStruct(pub(super) u8);

contextual! {
    pub(super) FromAnotherFile(OTHER): MyStruct = MyStruct(42);
}
