use std::vec::Vec;

pub struct NodeMan {
    mem : Vec<i32>,
}

impl NodeMan {
    const BLANK_ID : usize = 0x10000 as usize;
    const PASS_BASE : i32 = 1000;

    pub fn new() -> Self {
        NodeMan{
            mem : Vec::<i32>::new(),
        }
    }

    pub fn get_blank_id(&self) -> usize {
        Self::BLANK_ID
    }

    //計算途中で再確保が起こらないように必要サイズを確保しておく
    pub fn clear(&mut self, nblank : i32) {
		self.mem.clear();
		let size : i32 = (nblank + 2) * (nblank + 1) / 2;
        self.mem.reserve(size as usize);
	}

    // 打ち手を登録し、その場所を戻す
    pub fn set_move(&mut self, rmove : i32, pass : i32) -> usize {
        self.mem.push(rmove + pass * Self::PASS_BASE);
		self.mem.len() - 1
	}

    // idfrom以下の要素を取り除く
    pub fn unset_moves(&mut self, idfrom : usize) {
        if idfrom != Self::BLANK_ID {
            unsafe {
                self.mem.set_len(idfrom);
            }
        }
	}

    //idfrom以下をidto以下に上書きする
    pub fn transover_moves(&mut self, idfrom : usize, idto : usize) -> usize {
		if idto == Self::BLANK_ID {
			//上書きされる部分が存在しないので何もしない
			return idfrom;
        }
        assert!(idfrom > idto);
        let diff : usize = idfrom - idto;
        let len : usize = self.mem.len();
        //前につめる
        for id in idfrom .. len {
            self.mem[id - diff] = self.mem[id];
        }
        unsafe {
            //新たなsize
            self.mem.set_len(len - diff);
        }
		idto
	}

    pub fn len(&self) -> usize {
        self.mem.len()
    }

    pub fn get_move_list(&self) -> Vec<i32> {
		let mut list = Vec::<i32>::new();
		for i in 0 .. self.mem.len() {
			let pass : i32 = self.mem[i] / Self::PASS_BASE;
			let rmove : i32 = self.mem[i] % Self::PASS_BASE;
			if pass == 2 {
				list.push(-1);
				list.push(-1);
			} else {
				if pass == 1 {
					list.push(-1);
				}
				list.push(rmove);
			}
		}
		list
	}

}
