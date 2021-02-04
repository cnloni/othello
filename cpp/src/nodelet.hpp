#include <iostream>
#include <sstream>

// typedefs
struct Nodelet {
	uint64_t	bp{0};	//黒の駒配置
	uint64_t	wp{0};	//白の駒配置
	int turn{0};	//次の手番
	int alpha{-999};	//現在の最善結果
	int beta{999};	//最善結果の上限（この値より上回れば検索終了）
	Nodelet() = default;
	Nodelet(uint64_t bp0, uint64_t wp0, int turn0, int alpha0, int beta0) {
		set(bp0, wp0, turn0, alpha0, beta0);
	}
	void set(uint64_t bp0, uint64_t wp0, int turn0, int alpha0, int beta0) {
		bp = bp0;
		wp = wp0;
		turn = turn0;
		alpha = alpha0;
		beta = beta0;
	}
	std::string getString() {
    std::ostringstream oss;
  	oss << "(bp, wp, turn) = (" << bp << ", " << wp << ", " << turn << ")";
    oss << ", ";
    oss << "(alpha, beta) = (" << alpha << ", " << beta << ")";
    return oss.str();
  }
};
