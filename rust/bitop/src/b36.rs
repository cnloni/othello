use super::bitop::BitOpBase;

pub struct B36;

impl BitOpBase for B36 {
    const SIZE : i32 = 6;
	const CELLS : i32 = B36::SIZE * B36::SIZE;
	const MASK : u64 = 0x0000000FFFFFFFFF;
	const INITIAL_BP : u64 = 1081344;
	const INITIAL_WP : u64 = 2113536;
	const INITIAL_MOVE : i32 = 22;
    const RV_MASK0 : u64 = 0x000000079E79E79E;
    const RV_MASK1 : u64 = 0x000000003FFFFFC0;
    const RV_MASK2 : u64 = 0x000000001E79E780;

    fn get_upward(inc : i32, mask : u64, inturn : u64, opponent : u64) -> u64 {
        let w : u64 = opponent & mask;
        let mut t : u64 = w & (inturn << inc);
        t |= w & (t << inc);
        t |= w & (t << inc);
        t |= w & (t << inc);
        t |= w & (t << inc);
        t << inc
    }

    fn get_downward(inc : i32, mask : u64, inturn : u64, opponent : u64) -> u64 {
        let w : u64 = opponent & mask;
        let mut t : u64 = w & (inturn >> inc);
        t |= w & (t >> inc);
        t |= w & (t >> inc);
        t |= w & (t >> inc);
        t |= w & (t >> inc);
        t >> inc
    }

    fn reverse_upward(inc : i32, mask : u64, tmask : u64, inturn : u64, opponent : u64) -> u64 {
	  let w : u64 = opponent & mask;
	  let e1 : u64 = (tmask << inc) & w;
	  let e2 : u64 = (e1 << inc) & w;
	  let e3 : u64 = (e2 << inc) & w;
	  let e4 : u64 = (e3 << inc) & w;
	  let mut b1 : u64 = (inturn >> inc) & w;
      let mut b2 : u64 = (b1 >> inc) & w;
	  let mut rv : u64 = e4 & b1;
      b1 |= b2;
	  rv |= e3 & b1;
	  b2 = (b2 >> inc) & w;
      b1 |= b2;
	  rv |= e2 & b1;
	  b2 = (b2 >> inc) & w;
	  rv | (e1 & (b1 | b2))
	}

    fn reverse_downward(inc : i32, mask : u64, tmask : u64, inturn : u64, opponent : u64) -> u64 {
	  let w : u64 = opponent & mask;
	  let e1 : u64 = (tmask >> inc) & w;
	  let e2 : u64 = (e1 >> inc) & w;
	  let e3 : u64 = (e2 >> inc) & w;
	  let e4 : u64 = (e3 >> inc) & w;
	  let mut b1 : u64 = (inturn << inc) & w;
      let mut b2 : u64 = (b1 << inc) & w;
	  let mut rv : u64 = e4 & b1;
      b1 |= b2;
	  rv |= e3 & b1;
	  b2 = (b2 << inc) & w;
      b1 |= b2;
	  rv |= e2 & b1;
	  b2 = (b2 << inc) & w;
	  rv | (e1 & (b1 | b2))
	}
}
