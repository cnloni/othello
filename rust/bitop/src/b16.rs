use super::bitop::BitOpBase;

pub struct B16;

impl BitOpBase for B16 {
    const SIZE : i32 = 4;
	const CELLS : i32 = B16::SIZE * B16::SIZE;
	const MASK : u64 = 0x000000000000FFFF;
	const INITIAL_BP : u64 = 576;
	const INITIAL_WP : u64 = 1056;
	const INITIAL_MOVE : i32 = 11;
	const RV_MASK0 : u64 = 0x0000000000006666;
	const RV_MASK1 : u64 = 0x0000000000000FF0;
	const RV_MASK2 : u64 = 0x0000000000000660;

    #[inline(always)]
    fn get_upward(inc : i32, mask : u64, inturn : u64, opponent : u64) -> u64 {
        let w : u64 = opponent & mask;
        let mut t : u64 = w & (inturn << inc);
        t |= w & (t << inc);
        t |= w & (t << inc);
        t << inc
    }

    #[inline(always)]
    fn get_downward(inc : i32, mask : u64, inturn : u64, opponent : u64) -> u64 {
        let w : u64 = opponent & mask;
        let mut t : u64 = w & (inturn >> inc);
        t |= w & (t >> inc);
        t |= w & (t >> inc);
        t >> inc
    }

    #[inline(always)]
    fn reverse_upward(inc : i32, mask : u64, tmask : u64, inturn : u64, opponent : u64) -> u64 {
        let w : u64 = opponent & mask;
        let e1 : u64 = (tmask << inc) & w;
        let b1 : u64 = (inturn >> inc) & w;
        let rv : u64 = (e1 << inc) & b1 & w;
        let b2 : u64 = (b1 >> inc) & w;
        return rv | (e1 & (b1 | b2));
    }

    #[inline(always)]
    fn reverse_downward(inc : i32, mask : u64, tmask : u64, inturn : u64, opponent : u64) -> u64 {
        let w : u64 = opponent & mask;
        let e1 : u64 = (tmask >> inc) & w;
        let b1 : u64 = (inturn << inc) & w;
        let rv : u64 = (e1 >> inc) & b1 & w;
        let b2 : u64 = (b1 << inc) & w;
        return rv | (e1 & (b1 | b2));
    }
}
