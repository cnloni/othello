pub mod b16;
pub mod b36;
pub mod bitop;

#[cfg(test)]
mod tests {
    use crate::b36::B36;
    use crate::bitop::BitOp;
    use crate::bitop::BitOpBase;
    use rand::Rng;

    const COUNT : i32 = 100;
    #[test]
    fn test_candidates() {
        let inturn : u64 = 7913472;
        let opponent : u64 = 251666496;
        let candidates : u64 = 33286000768;
        let ccands : u64 = B36::get_candidates(inturn, opponent);
        assert_eq!(candidates, ccands);
    }
    fn bitcount_raw(q : u64) -> i32 {
        let mut c = 0;
        for m in 0..64 {
            if ((q >> m) & 1) == 1 {
                c += 1;
            }
        }
        c
    }
    fn lsb_raw(q : u64) -> i32 {
        let mut c = 0;
        for m in 0..64 {
            if ((q >> m) & 1) == 0 {
                c += 1;
            } else {
                break;
            }
        }
        c
    }
    #[test]
    fn test_bitcount() {
        let mut rng = rand::thread_rng();
        for _ in 0..COUNT {
            let r : u64 = rng.gen();
            let n : u64 = r & B36::MASK;
            let nbits1 = B36::bitcount(n);
            let nbits2 = bitcount_raw(n);
            assert_eq!(nbits1, nbits2);
            let lsbits1 = B36::lsb(n);
            let lsbits2 = lsb_raw(n);
            assert_eq!(lsbits1, lsbits2);
        }
    }
    #[test]
    fn test_reverse() {
        let inturn : u64 = 7913472;
        let opponent : u64 = 251666496;
        let mut candidates : u64 = B36::get_candidates(inturn, opponent);
        while candidates != 0 {
            let mut ti = inturn;
            let mut to = opponent;
            let bc : i32 = B36::bitcount(candidates);
            println!("bc = {}", bc);
            let rmove : i32 = B36::lsb(candidates);
            println!("rmove = {}", rmove);
            let reversed : i32 = B36::reverse(rmove, &mut ti, &mut to);
            assert!(reversed > 0);
            println!("reversed = {}", reversed);
            let tmask : u64 = 1u64 << rmove;
            candidates ^= tmask;
            println!("----");
        }
    }
}
