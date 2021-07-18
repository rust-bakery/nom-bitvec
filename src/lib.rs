use bitvec::prelude::*;

mod input;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct BSlice<'a, O: BitOrder, T: BitStore>(pub &'a BitSlice<O, T>);
pub struct BArray<O: BitOrder, T: BitStore>(pub BitArray<O, T>);
