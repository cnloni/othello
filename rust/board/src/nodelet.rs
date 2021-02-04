#[derive(Default)]
pub struct Nodelet {
	bp : u64,	//黒の駒配置
	wp : u64,	//白の駒配置
	turn : i32,    //次の手番
	alpha : i32,   //現在の最善結果
	beta : i32,    //最善結果の上限（この値より上回れば検索終了）
}

impl Nodelet {
    pub fn new() -> Self {
        Self{..Default::default()}
    }
    pub fn set(&mut self, bp : u64, wp : u64, turn : i32, alpha : i32, beta : i32) {
		self.bp = bp;
		self.wp = wp;
		self.turn = turn;
		self.alpha = alpha;
		self.beta = beta;
	}
}

impl std::fmt::Display for Nodelet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(bp, wp, turn) = ({}, {}, {}), (alpha, beta) = ({}, {})",
            self.bp, self.wp, self.turn, self.alpha, self.beta)
    }
}
