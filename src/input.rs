use bitvec::{prelude::*, slice::BitValIter};
use core::iter::Enumerate;
use core::ops::{Range, RangeFrom, RangeFull, RangeTo};
use nom::error::{ErrorKind, ParseError};
use nom::*;
/*use crate::lib::std::slice::Iter;
use crate::lib::std::str::from_utf8;
use crate::lib::std::str::CharIndices;
use crate::lib::std::str::Chars;*/

use crate::BSlice;

impl<'a, O, T> InputLength for BSlice<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore,
{
    #[inline]
    fn input_len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, O, T> Offset for BSlice<'a, O, T>
where
    O: BitOrder,
    T: BitStore,
{
    #[inline(always)]
    fn offset(&self, second: &Self) -> usize {
        second.0.offset_from(&self.0) as usize
    }
}

impl<'a, O> AsBytes for BSlice<'a, O, u8>
where
    O: BitOrder,
{
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self.0.as_raw_slice()
    }
}
/*
macro_rules! as_bytes_array_impls {
    ($($N:expr)+) => {
      $(
        impl<'a, O> AsBytes for &'a BArray<O, [u8; $N]>
        where O: BitOrder {
          #[inline(always)]
          fn as_bytes(&self) -> &[u8] {
            self.0.as_raw_slice()
          }
        }

        impl<O> AsBytes for BArray<O, [u8; $N]>
        where O: BitOrder {
          #[inline(always)]
          fn as_bytes(&self) -> &[u8] {
            self.0.as_raw_slice()
          }
        }
      )+
    };
  }


as_bytes_array_impls! {
    0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}*/

impl<'a, O, T> InputIter for BSlice<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore,
{
    type Item = bool;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = BitValIter<'a, O, T>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.0.iter().by_val()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.iter_elements().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.0.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.0.len()))
        }
    }
}

impl<'a, O, T> InputTake for BSlice<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore,
{
    #[inline]
    fn take(&self, count: usize) -> Self {
        BSlice(&self.0[..count])
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (a, b) = self.0.split_at(count);
        (BSlice(b), BSlice(a))
    }
}

impl<'a, O, T> InputTakeAtPosition for BSlice<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore,
{
    type Item = bool;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0
            .iter()
            .by_val()
            .position(predicate)
            .map(|i| {
                let (a, b) = self.0.split_at(i);
                (BSlice(a), BSlice(b))
            })
            .ok_or_else(|| Err::Incomplete(Needed::new(1)))
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.0.iter().by_val().position(predicate) {
            Some(0) => {
                let s = BSlice(self.0.split_at(0).1);
                Err(Err::Error(E::from_error_kind(s, e)))
            }
            Some(i) => Ok({
                let (a, b) = self.0.split_at(i);
                (BSlice(a), BSlice(b))
            }),
            None => Err(Err::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0
            .iter()
            .position(|b| predicate(*b))
            .map(|i| {
                let (a, b) = self.0.split_at(i);
                (BSlice(a), BSlice(b))
            })
            .or_else(|| {
                let s = BSlice(self.0.split_at(0).1);
                Some((s, BSlice(BitSlice::empty())))
            })
            .ok_or_else(|| unreachable!())
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.0.iter().by_val().position(predicate) {
            Some(0) => {
                let s = BSlice(self.0.split_at(0).1);
                Err(Err::Error(E::from_error_kind(s, e)))
            }
            Some(i) => Ok({
                let (a, b) = self.0.split_at(i);
                (BSlice(a), BSlice(b))
            }),
            None => {
                if self.0.is_empty() {
                    let s = BSlice(self.0.split_at(0).1);
                    Err(Err::Error(E::from_error_kind(s, e)))
                } else {
                    let s = BSlice(self.0.split_at(0).1);
                    Ok((s, BSlice(BitSlice::empty())))
                }
            }
        }
    }
}

impl<'a, 'b, O1, O2, T1, T2> Compare<BSlice<'b, O2, T2>> for BSlice<'a, O1, T1>
where
    O1: BitOrder,
    O2: BitOrder,
    T1: 'a + BitStore,
    T2: 'a + BitStore,
{
    #[inline]
    fn compare(&self, other: BSlice<'b, O2, T2>) -> CompareResult {
        match self.0.iter().zip(other.0.iter()).position(|(a, b)| a != b) {
            Some(_) => CompareResult::Error,
            None => {
                if self.0.len() >= other.0.len() {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }

    #[inline(always)]
    fn compare_no_case(&self, other: BSlice<'b, O2, T2>) -> CompareResult {
        self.compare(other)
    }
}

impl<'a, O, T> FindToken<bool> for BSlice<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore,
{
    fn find_token(&self, token: bool) -> bool {
        self.0.iter().by_val().any(|i| i == token)
    }
}

impl<'a, O, T> FindToken<(usize, bool)> for BSlice<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore,
{
    fn find_token(&self, token: (usize, bool)) -> bool {
        self.0.iter().by_val().enumerate().any(|i| i == token)
    }
}

impl<'a, 'b, O1, O2, T1, T2> FindSubstring<BSlice<'b, O2, T2>> for BSlice<'a, O1, T1>
where
    O1: BitOrder,
    O2: BitOrder,
    T1: 'a + BitStore,
    T2: 'b + BitStore,
{
    fn find_substring(&self, substr: BSlice<O2, T2>) -> Option<usize> {
        if substr.0.len() > self.0.len() {
            return None;
        }

        if substr.0.is_empty() {
            return Some(0);
        }

        self.0
            .windows(substr.0.len())
            .position(|window| window == substr.0)
    }
}

macro_rules! impl_fn_slice {
    ( $ty:ty ) => {
        fn slice(&self, range: $ty) -> Self {
            BSlice(&self.0[range])
        }
    };
}

macro_rules! slice_range_impl {
    ( BSlice, $ty:ty ) => {
        impl<'a, O, T> Slice<$ty> for BSlice<'a, O, T>
        where
            O: BitOrder,
            T: BitStore,
        {
            impl_fn_slice!($ty);
        }
    };
}

macro_rules! slice_ranges_impl {
    ( BSlice ) => {
        slice_range_impl! {BSlice, Range<usize>}
        slice_range_impl! {BSlice, RangeTo<usize>}
        slice_range_impl! {BSlice, RangeFrom<usize>}
        slice_range_impl! {BSlice, RangeFull}
    };
}

slice_ranges_impl! {BSlice}

#[cfg(feature = "alloc")]
impl<'a, O, T> ExtendInto for BSlice<'a, O, T>
where
    O: BitOrder,
    T: BitStore,
{
    type Item = bool;
    type Extender = BitVec<O, T>;

    #[inline]
    fn new_builder(&self) -> BitVec<O, T> {
        BitVec::new()
    }

    #[inline]
    fn extend_into(&self, acc: &mut Self::Extender) {
        acc.extend(self.0.iter());
    }
}
