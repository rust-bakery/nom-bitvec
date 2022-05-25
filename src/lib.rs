//! This crate provides input types for [nom parser combinators](https://crates.io/crates/nom)
//! using [bitvec](https://crates.io/crates/bitvec).
//! With those, you can use common nom combinators directly on streams of bits.
//!
//! ## Example
//!
//! ```rust,ignore
//! let data = [0xA5u8, 0x69, 0xF0, 0xC3];
//! let bits = data.view_bits::<Msb0>();
//!
//! fn parser(bits: &BitSlice<Msb0, u8>) -> IResult<&BitSlice<Msb0, u8>, &BitSlice<Msb0, u8>> {
//!   tag(bits![1, 0, 1, 0])(bits)
//! }
//!
//! assert_eq!(parser(bits), Ok((&bits[..4], &bits[4..])));
//! ```
use bitvec::prelude::*;

mod input;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct BSlice<'a, T: BitStore, O: BitOrder>(pub &'a BitSlice<T, O>);
pub struct BArray<T: BitStore, O: BitOrder>(pub BitArray<T, O>);

impl<'a, T: BitStore, O: BitOrder> From<&'a BitSlice<T, O>> for BSlice<'a, T, O> {
    fn from(slice: &'a BitSlice<T, O>) -> Self {
        Self(slice)
    }
}

impl<T: BitStore, O: BitOrder> From<BitArray<T, O>> for BArray<T, O> {
    fn from(slice: BitArray<T, O>) -> Self {
        Self(slice)
    }
}
