use bitvec::prelude::*;
use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    IResult,
};
use nom_bitvec::BSlice;

#[test]
fn parse_bitstream() {
    let data = [0xA5u8, 0x69, 0xF0, 0xC3];
    let bits = data.view_bits::<Msb0>();

    fn parser(bits: BSlice<u8, Msb0>) -> IResult<BSlice<u8, Msb0>, BSlice<u8, Msb0>> {
        tag(BSlice(bits![1, 0, 1, 0]))(bits)
    }

    assert_eq!(
        parser(BSlice(bits)),
        Ok((BSlice(&bits[4..]), BSlice(&bits[..4])))
    );
}

#[test]
fn parse_bitstream_map() {
    let data = [0b1000_0000];
    let bits = data.view_bits::<Msb0>();

    fn parser(bits: BSlice<u8, Msb0>) -> IResult<BSlice<u8, Msb0>, bool> {
        map(take(1_u8), |val: BSlice<u8, Msb0>| val[0])(bits)
    }

    assert_eq!(parser(BSlice(bits)), Ok((BSlice(&bits[1..]), true)));
}
