use std::marker::PhantomData;
use std::mem;
use std::time::Instant;
use bitop::bitop::BitOp;
use bitop::bitop::BitOpBase;
use crate::nodeman::NodeMan;
use crate::nodelet::Nodelet;

pub struct Board<T> {
    node_count : u64,
    final_result : i32,
    initial_nodelet : Nodelet,
    final_nodelet : Nodelet,
    elapsed : f64,
    nodeman : NodeMan,
    _marker : PhantomData<T>,
}

impl<T: BitOpBase + BitOp> Board<T> {
    pub fn new() -> Self {
        Board{
            node_count : 0u64,
            final_result : -1,
            initial_nodelet : Nodelet::new(),
            final_nodelet : Nodelet::new(),
            elapsed : 0.0,
            nodeman : NodeMan::new(),
            _marker : PhantomData::<T>
        }
    }

    pub fn get_best_result_from_start(&mut self) -> i32 {
        self.get_best_result_with_ab(T::INITIAL_BP, T::INITIAL_WP, 0, -(T::CELLS + 1), T::CELLS + 1)
    }

    pub fn get_best_result(&mut self, bp : u64, wp : u64, turn : i32) -> i32 {
        self.get_best_result_with_ab(bp, wp, turn, -(T::CELLS + 1), T::CELLS + 1)
    }

    pub fn get_best_result_with_ab(&mut self, bp : u64, wp : u64, turn : i32, alpha : i32, beta : i32) -> i32 {
        //打ち手登録マネジャを初期化する（clear()に残り枠数を送る）
    	self.nodeman.clear(T::CELLS - T::bitcount(bp | wp));
        self.node_count = 0;
        let mut next_alpha : i32 = alpha;
        let mut next_beta : i32 = beta;
        let mut next_turn : i32 = turn;
        let mut inturn : u64 = if turn == 0 { bp } else { wp };
      	let mut opponent : u64 = if turn == 0 { wp } else { bp };
        let initial_stage : bool = (bp == T::INITIAL_BP) && (wp == T::INITIAL_WP);

        self.initial_nodelet.set(bp, wp, turn, alpha, beta);

        // 時間計測開始
        let start = Instant::now();

        if  initial_stage {
            //初期盤面の時は初手が定められているので選択せず決め打ちする
            T::reverse(T::INITIAL_MOVE, &mut inturn, &mut opponent);
            self.nodeman.set_move(T::INITIAL_MOVE, 0);
            next_alpha = -beta;
            next_beta = -alpha;
            next_turn = 1 - turn;
            mem::swap(&mut inturn, &mut opponent);
        }

        let result : i32 = self.calc_node(&mut next_turn, &mut inturn, &mut opponent,
            next_alpha, next_beta, 0);

        // 時間計測終了
        let duration = start.elapsed();
        self.elapsed = duration.subsec_micros() as f64 * 1e-6 + duration.as_secs() as f64;

        let final_bp = if next_turn == 0 { inturn } else { opponent };
        let final_wp = if next_turn == 0 { opponent } else { inturn };
        let final_alpha = if initial_stage { -beta } else { result };
        let final_beta = if initial_stage { -result } else { beta };

        //黑番から見た最終結果
        if initial_stage {
            self.final_result = if turn == 0 { -result } else { result };
        } else {
            self.final_result = if turn == 0 { result } else { -result };
        }
        self.final_nodelet.set(final_bp, final_wp, next_turn, final_alpha, final_beta);
        self.final_result
    }

    fn calc_node(&mut self, pturn : &mut i32, pinturn : &mut u64, popponent : &mut u64,
    		alpha : i32, beta : i32, current_pass : i32) -> i32 {
        self.node_count += 1;
    	if *pinturn == 0 {
    		//途中で駒が無くなったら負け
    		return - T::SIZE;
    	}
    	//どちらの駒も置かれていない場所
    	let blank : u64 = (!(*pinturn | *popponent)) & T::MASK;
    	if blank == 0 {
    		//全マスが埋まったので終了
    		//手番側から見た得点結果を戻す
    		return T::bitcount(*pinturn) - T::bitcount(*popponent);
    	}

    	//全候補手
    	let mut candidates : u64 = T::get_candidates(*pinturn, *popponent);

        if candidates == 0 {
            //打つ場所がない
            if current_pass == 0 {
                //一回パス（手番を入れ換える）
                let mut inturn : u64 = *pinturn;
                let mut opponent : u64 = *popponent;
                let mut next_turn : i32 = 1 - *pturn;

                let result : i32 = - self.calc_node(&mut next_turn, &mut opponent, &mut inturn, -beta, -alpha, 1);

                *pinturn = opponent;
                *popponent = inturn;
                *pturn = next_turn;
                return result;
            } else {
                //連続パスで終了
                //2回パスを登録
                self.nodeman.set_move(0, 2);
                //残りのマスは勝者のものとする
                let tt : i32 = T::bitcount(*pinturn);
                let to : i32 = T::bitcount(*popponent);
                if tt > to {
                    return T::CELLS - 2 * to;
                } else if tt < to {
                    return 2 * tt - T::CELLS;
                } else {
                    return 0;
                }
            }
        }
		//打ち手が1個以上ある
		let mut best_inturn : u64 = *pinturn;
		let mut best_opponent : u64 = *popponent;
		let mut best_turn : i32 = *pturn;
        let mut current_alpha = alpha;
        //最初は最良手順が存在しない
		let mut best_id : usize = self.nodeman.get_blank_id();

		while candidates != 0 {
			//候補手のうちから打ち手を選択する
			let rmove : i32 = T::lsb(candidates);
			//選択された打ち手だけを盤上に配置する
			let tmask : u64 = 1u64 << rmove;
			//選択された打ち手を候補手から削除する
			candidates ^= tmask;

            //盤面を初期化
    		let mut inturn : u64 = *pinturn;
    		let mut opponent : u64 = *popponent;
    		let mut next_turn : i32 = 1 - *pturn;

			//打ち手に対して次の盤面を得る（挟まれた駒を反転する）
			T::reverse(rmove, &mut inturn, &mut opponent);

            //打ち手以下を辿る（当然passではない）
			//打ち手を登録する
			let id : usize = self.nodeman.set_move(rmove, current_pass);

			//次の手番は入れ替わる
			let result : i32 = - self.calc_node(&mut next_turn, &mut opponent, &mut inturn, -beta, -current_alpha, 0);

			if result > current_alpha {
				//最良結果であったので、最良結果を更新
				current_alpha = result;
                //最良結果に対する最終盤面と最終手番を更新
				best_inturn = opponent;
				best_opponent = inturn;
				best_turn = next_turn;
                //id以下が最良結果の手順だから、これを最良手順とする
				best_id = self.nodeman.transover_moves(id, best_id);
                if current_alpha >= beta {
                    //これ以上候補手を探さない
                    //親段においてこの手順の結果が最良にならないことが確定した
                    self.nodeman.unset_moves(best_id);
                    return current_alpha;
                }
			} else {
				//結果が最良でないので手順を破棄する
				self.nodeman.unset_moves(id);
			}
		}
		// 最良の結果の属性（現在手番側最終配置、現在相手側最終配置、最終手番）を設定する
		*pinturn = best_inturn;
		*popponent = best_opponent;
		*pturn = best_turn;
		current_alpha
    }

    pub fn get_move_list_string(&self) -> String {
        let moves = self.nodeman.get_move_list();
        let mut ret : String = String::new();
        for rmove in moves {
            if !ret.is_empty() {
              ret.push(' ');
            }
            ret.push_str(&Self::positive_to_cell(rmove));
        }
        ret
    }

    pub fn get_initial(&self) -> &Nodelet {
        &self.initial_nodelet
    }

    pub fn get_final(&self) -> &Nodelet {
        &self.final_nodelet
    }

    pub fn get_move_list(&self) -> Vec<i32> {
        self.nodeman.get_move_list()
    }

    pub fn get_elapsed(&self) -> f64 {
        self.elapsed
    }

    fn positive_to_cell(positive : i32) -> String {
        if positive < 0 {
        	return "pa".to_string();
        }
        let r : i32 = positive / T::SIZE;
        let c : i32 = positive % T::SIZE;
        let row : char = char::from(0x31 + r as u8);
        let col : char = char::from(0x61 + c as u8);
        let mut ret = String::from(col);
        ret.push(row);
        ret
    }
}
