use core::cmp;

//盤のサイズに依存する定位数と関数
pub trait BitOpBase {
    const SIZE : i32;
	const CELLS : i32;
	const MASK : u64;
	const INITIAL_BP : u64;
	const INITIAL_WP : u64;
	const INITIAL_MOVE : i32;
    const RV_MASK0 : u64;
    const RV_MASK1 : u64;
    const RV_MASK2 : u64;
    fn get_upward(inc : i32, mask : u64, inturn : u64, opponent : u64) -> u64;
    fn get_downward(inc : i32, mask : u64, inturn : u64, opponent : u64) -> u64;
    fn reverse_upward(inc : i32, mask : u64, tmask : u64, inturn : u64, opponent : u64) -> u64;
    fn reverse_downward(inc : i32, mask : u64, tmask : u64, inturn : u64, opponent : u64) -> u64;
}

//盤のサイズに依存しない関数
pub trait BitOp {
    fn get_candidates(inturn : u64, opponent : u64) -> u64;
    fn reverse(rmove : i32, pt : &mut u64, po : &mut u64) -> i32;
    // u64の全64bitのうち1であるbitの総数を求める
    // cpuの組み込み関数をコールする
    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn bitcount(x: u64) -> i32 {
        unsafe {
            std::arch::x86_64::_popcnt64(x as i64)
        }
    }
    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn lsb(x: u64) -> i32 {
        unsafe {
            std::arch::x86_64::_tzcnt_u64(x) as i32
        }
    }
}

impl<T: BitOpBase> BitOp for T {
	// 局面に対する候補手を求める
	// inturn:手番の盤面
	// opponent:相手番の盤面
	// 戻り値:候補手の盤面（全候補手を盤面に配置する）
	fn get_candidates(inturn : u64, opponent : u64) -> u64 {
        let candidates : u64 =
            T::get_upward(1, T::RV_MASK0, inturn, opponent)
            | T::get_downward(1, T::RV_MASK0, inturn, opponent)
            | T::get_upward(T::SIZE, T::RV_MASK1, inturn, opponent)
            | T::get_downward(T::SIZE, T::RV_MASK1, inturn, opponent)
            | T::get_upward(T::SIZE - 1, T::RV_MASK2, inturn, opponent)
            | T::get_downward(T::SIZE - 1, T::RV_MASK2, inturn, opponent)
            | T::get_upward(T::SIZE + 1, T::RV_MASK2, inturn, opponent)
            | T::get_downward(T::SIZE + 1, T::RV_MASK2, inturn, opponent);
		(!(inturn | opponent)) & candidates
	}
    // 打ち手前の盤面を打ち手後の盤面に変更する
	// n:打ち手
	// *pt:手番の盤面
	// *po:相手番の盤面
	// 戻り値:返したコマ数
	fn reverse(rmove : i32, pt : &mut u64, po : &mut u64) -> i32 {
		let x1 : i32 = rmove % T::SIZE;
		let x2 : i32 = (T::SIZE - 1) - x1;
		let y1 : i32 = rmove / T::SIZE;
		let y2 : i32 = (T::SIZE - 1) - y1;

        let tmask : u64 = 1u64 << rmove;
        let mut rev : u64 = 0;

		//右
		if x2 >= 2 {
            rev |= T::reverse_upward(1, T::RV_MASK0, tmask, *pt, *po);
		}
        //左
        if x1 >= 2 {
            rev |= T::reverse_downward(1, T::RV_MASK0, tmask, *pt, *po);
        }
        //下
		if y2 >= 2 {
            rev |= T::reverse_upward(T::SIZE, T::RV_MASK1, tmask, *pt, *po);
		}
        //上
        if y1 >= 2 {
            rev |= T::reverse_downward(T::SIZE, T::RV_MASK1, tmask, *pt, *po);
        }
        //左斜下
        if cmp::min(x1, y2) >= 2 {
            rev |= T::reverse_upward(T::SIZE - 1, T::RV_MASK2, tmask, *pt, *po);
        }
        //右斜上
        if cmp::min(x2, y1) >= 2 {
            rev |= T::reverse_downward(T::SIZE - 1, T::RV_MASK2, tmask, *pt, *po);
        }
        //右斜下
        if cmp::min(x2, y2) >= 2 {
            rev |= T::reverse_upward(T::SIZE + 1, T::RV_MASK2, tmask, *pt, *po);
        }
        //左斜上
        if cmp::min(x1, y1) >= 2 {
            rev |= T::reverse_downward(T::SIZE + 1, T::RV_MASK2, tmask, *pt, *po);
        }
        *pt |= rev | tmask;
        *po ^= rev;
		//返した駒の数
		T::bitcount(rev)
	}
}
