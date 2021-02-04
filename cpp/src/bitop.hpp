#pragma once

#include <string>
#include <sstream>

using namespace std;

// constants
constexpr static uint64_t ONE = 1;

//
// クラスに依存しないインラインのビット処理関数
// inline functions
inline int bitcount(uint64_t m) {
	return __builtin_popcountll((int64_t)m);
}

inline int lsb(uint64_t m) {
		return __builtin_ctzll((int64_t)m);
}

// template classes
template <typename CTO>
struct BitOp {
	//
	// クラスで共通なビット処理関数
	//
	static bool checkCandidates(uint64_t inturn, uint64_t opponent, uint64_t candidates) {
		return candidates == getCandidates(inturn, opponent);
	}
	// 局面に対する候補手を求める
	// inturn:手番の盤面
	// opponent:相手番の盤面
	// 戻り値:候補手の盤面（全候補手を盤面に配置する）
	static uint64_t getCandidates(uint64_t inturn, uint64_t opponent) {
		uint64_t candidates = CTO::getUpward(1, CTO::RV_MASK0, inturn, opponent);
		candidates |= CTO::getDownward(1, CTO::RV_MASK0, inturn, opponent);
		candidates |= CTO::getUpward(CTO::SIZE, CTO::RV_MASK1, inturn, opponent);
		candidates |= CTO::getDownward(CTO::SIZE, CTO::RV_MASK1, inturn, opponent);
		candidates |= CTO::getUpward(CTO::SIZE - 1, CTO::RV_MASK2, inturn, opponent);
		candidates |= CTO::getDownward(CTO::SIZE - 1, CTO::RV_MASK2, inturn, opponent);
		candidates |= CTO::getUpward(CTO::SIZE + 1, CTO::RV_MASK2, inturn, opponent);
		candidates |= CTO::getDownward(CTO::SIZE + 1, CTO::RV_MASK2, inturn, opponent);
		return (~(inturn | opponent)) & candidates;
	}
	// 打ち手前の盤面を打ち手後の盤面に変更する
	// n:打ち手
	// *pt:手番の盤面
	// *po:相手番の盤面
	// 戻り値:返したコマ数
	static int reverse(int move, uint64_t *pt, uint64_t *po) {
		int x1 = move % CTO::SIZE;
		int x2 = (CTO::SIZE - 1) - x1;
		int y1 = move / CTO::SIZE;
		int y2 = (CTO::SIZE - 1) - y1;

		uint64_t tmask = ONE << move;
	  uint64_t rev{0};

		//右
		if (x2 >= 2) {
	    rev |= CTO::reverseUpward(1, CTO::RV_MASK0, tmask, *pt, *po);
		}
	  //左
	  if (x1 >= 2) {
	    rev |= CTO::reverseDownward(1, CTO::RV_MASK0, tmask, *pt, *po);
	  }
	  //下
		if (y2 >= 2) {
	    rev |= CTO::reverseUpward(CTO::SIZE, CTO::RV_MASK1, tmask, *pt, *po);
		}
	  //上
	  if (y1 >= 2) {
	    rev |= CTO::reverseDownward(CTO::SIZE, CTO::RV_MASK1, tmask, *pt, *po);
	  }
	  //左斜下
		if (min(x1, y2) >= 2) {
	    rev |= CTO::reverseUpward(CTO::SIZE - 1, CTO::RV_MASK2, tmask, *pt, *po);
		}
	  //右斜上
	  if (min(x2, y1) >= 2) {
	    rev |= CTO::reverseDownward(CTO::SIZE - 1, CTO::RV_MASK2, tmask, *pt, *po);
	  }
	  //右斜下
		if (min(x2, y2) >= 2) {
	    rev |= CTO::reverseUpward(CTO::SIZE + 1, CTO::RV_MASK2, tmask, *pt, *po);
		}
	  //左斜上
	  if (min(x1, y1) >= 2) {
	    rev |= CTO::reverseDownward(CTO::SIZE + 1, CTO::RV_MASK2, tmask, *pt, *po);
	  }
	  *pt |= rev | tmask;
	  *po ^= rev;
		//返した駒の数
		return bitcount(rev);
	}
};
