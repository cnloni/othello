#include <chrono>
#include "nodelet.hpp"
#include "bitop.hpp"
#include "tool.hpp"
#include "nodeman.hpp"

using namespace std::chrono;

template <typename CTO>
class Board {
protected:
  Nodelet initial;
  Nodelet final;
  double elapsed; //計算時間（秒単位）
	uint64_t nodeCount{0};
  int finalResult{0}; //getBestResult()の結果
	NodeManager<CTO> nodeman{};
public:
	Board() = default;
	//getter / setter
  //計算にかかった時間を取得する
  double getElapsedTime() {
    return elapsed;
  }
  uint64_t getNodeCount() {
    return nodeCount;
  }
  string getInitialNodeString() {
    return initial.getString();
  }
  string getFinalNodeString() {
    return final.getString();
  }
  uint64_t getFinalBlack() {
    return final.bp;
  }
  uint64_t getFinalWhite() {
    return final.wp;
  }
  string getFinalBoardString() {
    return Tool<CTO>::getBoardString(final.bp, final.wp);
  }
  int getBestResult(
    uint64_t bp = CTO::INITIAL_BP,
    uint64_t wp = CTO::INITIAL_WP,
    int turn = 0,
    int alpha = -(CTO::CELLS + 1),
    int beta = CTO::CELLS + 1
  );
  vector<int> getMoveList() {
    return nodeman.getList();
  }
  string getMoveListString() {
    vector<int> moves = nodeman.getList();
    ostringstream oss;
  	for(auto& move : moves) {
  		oss << Tool<CTO>::positiveToCell(move) << ' ';
  	}
    return oss.str();
  }
  int calcNode(int *pTurn, uint64_t *pInturn, uint64_t *pOpponent,
  		int alpha, int beta, int pass);
};

template <typename CTO>
int Board<CTO>::getBestResult(
    uint64_t bp, uint64_t wp, int turn, int alpha, int beta)
{
  //初期状態の設定
  initial.set(bp, wp, turn, alpha, beta);
	//打ち手登録マネジャを初期化する
	nodeman.clear(bp, wp);
  nodeCount = 0;

  int nextAlpha = alpha;
  int nextBeta = beta;
  int nextTurn = turn;
  bool initialStage = (bp == CTO::INITIAL_BP && wp == CTO::INITIAL_WP);

  auto startTime = system_clock::now();
	if (initialStage) {
		//初期盤面の時は初手が定められているので選択せず決め打ちする
		BitOp<CTO>::reverse(CTO::INITIAL_MOVE, &bp, &wp);
		//初手を登録する
		nodeman.setMove(CTO::INITIAL_MOVE, 0);
    nextAlpha = -beta;
    nextBeta = -alpha;
    nextTurn = 1 - turn;
	}
  uint64_t inturn = nextTurn?wp:bp;
	uint64_t opponent = nextTurn?bp:wp;

	int result = -calcNode(&nextTurn, &inturn, &opponent, nextAlpha, nextBeta, 0);

  auto endTime = system_clock::now();
  elapsed = (double)duration_cast<microseconds>(endTime - startTime).count() * 1e-6;

  //黑番から見た最終結果
  if (initialStage) {
    result = -result;
  }
  finalResult = turn?result:-result;
  int finalAlpha = initialStage?-beta:result;
  int finalBeta = initialStage?-result:beta;

  //終了状態の設定
  final.set(
	 nextTurn?opponent:inturn,
   nextTurn?inturn:opponent,
	 nextTurn,
   finalAlpha,
   finalBeta
  );

	//黑番から見た獲得駒数の差
	//2回パスによる終了があるため、
	//finalResult=bitcount(finalBp)-bitcount(finalWp)とは限らない
	//uint64_t nodeCount = Board<CTO>::getNodeCount();
	//cout << "count = " << nodeCount << endl;;
	return finalResult;
}

template <typename CTO>
int Board<CTO>::calcNode(int *pTurn, uint64_t *pInturn, uint64_t *pOpponent,
		int alpha, int beta, int currentPass) {
  nodeCount++;
	if (!*pInturn) {
		//途中で駒が無くなったら負け
		return - CTO::SIZE;
	}
	//どちらの駒も置かれていない場所
	uint64_t blank = (~(*pInturn | *pOpponent)) & CTO::MASK;
	if (!blank) {
		//全マスが埋まったので終了
		//手番側から見た得点結果を戻す
		return bitcount(*pInturn) - bitcount(*pOpponent);
	}

	//全候補手
	uint64_t candidates = BitOp<CTO>::getCandidates(*pInturn, *pOpponent);

  if (candidates == 0) {
		//打つ場所がない
		if (currentPass == 0) {
			//一回パス（手番を入れ換える）
			int nextTurn = 1 - *pTurn;
			uint64_t inturn = *pInturn;
			uint64_t opponent = *pOpponent;
			int result = -calcNode(&nextTurn, &opponent, &inturn, -beta, -alpha, 1);
			*pTurn = nextTurn;
			*pInturn = opponent;
			*pOpponent = inturn;
			return result;
		} else {
			//連続パスで終了
			//2回パスを登録
			nodeman.setMove(0, 2);
			//残りのマスは勝者のものとする
			int tt = bitcount(*pInturn);
			int to = bitcount(*pOpponent);
			if (tt > to) {
				return CTO::CELLS - 2 * to;
			} else if (tt < to) {
				return 2 * tt - CTO::CELLS;
			} else {
				return 0;
			}
		}
  }

	//打ち手が1個以上ある
	uint64_t bestInturn;
	uint64_t bestOpponent;
	int bestTurn;
	//初期化
	uint64_t inturn = *pInturn;
	uint64_t opponent = *pOpponent;
	int nextTurn = 1 - *pTurn;
	int bestId{NodeManager<CTO>::BLANK_ID};

	while(candidates) {
		//候補手をひとつ選択する
		int move = lsb(candidates);
		//候補手の盤上の配置
		uint64_t tmask = ONE << move;
		//選択した打ち手を削除
		candidates ^= tmask;

		//打ち手に対して双方の駒を反転する
		BitOp<CTO>::reverse(move, &inturn, &opponent);

		//打ち手を登録する
		int id = nodeman.setMove(move, currentPass);

		//打ち手以下を辿る（当然passではない）
		//次の手番は入れ替わる
		int result = -calcNode(&nextTurn, &opponent, &inturn, -beta, -alpha, 0);
    if (result >= beta) {
      //これ以上候補手を探さない
      //親段においてこの手順の結果が最良にならないことが確定した
      //nodeman.unsetMoves(id);
      return result;
    }
		if (result > alpha) {
			//最良結果であった
			//最良結果を更新
			alpha = result;
			//最良結果に対する最終盤面と最終手番
			bestInturn = opponent;
			bestOpponent = inturn;
			bestTurn = nextTurn;
			//id以下が最良結果の手順だから、これを最良手順とする
			bestId = nodeman.transoverMoves(id, bestId);
		} else {
			//結果が最良でないので手順を破棄する
			nodeman.unsetMoves(id);
		}
		//盤面が更新されているので再初期化
		inturn = *pInturn;
		opponent = *pOpponent;
		nextTurn = 1 - *pTurn;
	}
	// 最良の結果の属性（現在手番側最終配置、現在相手側最終配置、最終手番）を設定する
	*pInturn = bestInturn;
	*pOpponent = bestOpponent;
	*pTurn = bestTurn;
	return alpha;
}
