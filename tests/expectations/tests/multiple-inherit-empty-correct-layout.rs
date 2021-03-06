/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Foo ) ));
    assert_eq! (::std::mem::align_of::<Foo>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Foo ) ));
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Bar ) ));
    assert_eq! (::std::mem::align_of::<Bar>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Bar ) ));
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Baz {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(::std::mem::size_of::<Baz>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Baz ) ));
    assert_eq! (::std::mem::align_of::<Baz>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Baz ) ));
}
impl Clone for Baz {
    fn clone(&self) -> Self { *self }
}
