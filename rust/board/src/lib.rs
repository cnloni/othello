pub mod board;
pub mod nodeman;
pub mod nodelet;

#[cfg(test)]
mod tests {
    use crate::nodeman::NodeMan;
    use crate::nodelet::Nodelet;
    use crate::board::Board;
    use bitop::b16::B16;
    #[test]
    fn test_board_16() {
        let mut board : Board<B16> = Board::<B16>::new();
        //初手から実行
        let ret1 : i32 = board.get_best_result_from_start();
        assert_eq!(ret1, -10);

        //第2手から実行
        let ret2 : i32 = board.get_best_result(3648, 32, 1);
        assert_eq!(ret2, -10);
    }
    #[test]
    fn test_nodeman() {
        let mut man : NodeMan = NodeMan::new();
        man.clear(10);
        let mut idlast : usize = man.set_move(11, 0);
        man.set_move(12, 0);
        man.set_move(13, 1);
        man.set_move(14, 0);
        idlast = man.set_move(15, 1);
        assert!(man.len() == idlast + 1);

        man.unset_moves(3);
        assert!(man.len() == 3);

        let moves = man.get_move_list();
        assert!(moves.len() == 4);
    }
/*
    #[test]
    fn test_nodelet() {
        let mut nodelet : Nodelet = Nodelet::new();
        assert!(nodelet.get_black() == 0u64);
    }
*/
}
