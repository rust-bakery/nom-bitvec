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
pub struct BSlice<'a, O: BitOrder, T: BitStore>(pub &'a BitSlice<O, T>);
pub struct BArray<O: BitOrder, T: BitStore>(pub BitArray<O, T>);
