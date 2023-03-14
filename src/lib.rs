pub mod alg;

mod axs;

pub use axs::Dimension;


pub trait Get<'a, Idx: ?Sized> {
    type Output: ?Sized;
    fn get(&'a self, index: Idx) -> Self::Output;
}

pub trait GetMut<'a, Idx: ?Sized> {
    type Output: ?Sized;
    fn get_mut(&'a mut self, index: Idx) -> Self::Output;
}