pub use mutification_derive::ToMut;

pub trait ToMut {
    fn to_mut<'mutification_to_mut>(&'mutification_to_mut self) -> &'mutification_to_mut mut Self;
}
