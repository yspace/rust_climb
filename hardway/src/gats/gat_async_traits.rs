#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]


// pub trait KvIterator {
//     type NextFuture: Future<Output = Option<(&[u8], &[u8])>>;

//     /// Get the next item from the iterator.
//     fn next(&mut self) -> Self::NextFuture;
// }

use futures::Future;

pub trait KvIterator {
    type NextFuture<'a>: Future<Output = Option<(&'a [u8], &'a [u8])>>
    where
        Self: 'a;

    /// Get the next item from the iterator.
    fn next(&mut self) -> Self::NextFuture<'_>;
}


pub struct TestIterator {
    idx: usize,
}
// FIXME: error[E0658]: `impl Trait` in associated types is unstable
// 需要 nightly模式！
// impl KvIterator for TestIterator {
//     type NextFuture<'a> = impl Future<Output = Option<(&'a [u8], &'a [u8])>>;
//     fn next(&mut self) -> Self::NextFuture<'_> {
//         async move { Some((b"key".as_slice(), b"value".as_slice())) }
//     }
// }

#[test]
fn test_it(){

}