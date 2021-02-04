#include <iostream>
#include "b16.hpp"
#include "b36.hpp"
#include "board.hpp"

using namespace std;

int execute16(uint64_t bp, uint64_t wp, int turn, int alpha, int beta) {
	Board<B16> board;
	int result = board.getBestResult(bp, wp, turn, alpha, beta);

	cout << board.getFinalBoardString() << endl;
	cout << "Initial = " << board.getInitialNodeString() << endl;
	cout << "Final = " << board.getFinalNodeString() << endl;
	cout << "Result = " << result << endl;
	cout << "Moves = " << board.getMoveListString() << endl;
	cout << "Count = " << board.getNodeCount() << endl;
	cout << "Elapsed = " << board.getElapsedTime() << endl;
	return 0;
}

int execute36(uint64_t bp, uint64_t wp, int turn, int alpha, int beta) {
	Board<B36> board;
	int result = board.getBestResult(bp, wp, turn, alpha, beta);

	cout << board.getFinalBoardString() << endl;
	cout << "Initial = " << board.getInitialNodeString() << endl;
	cout << "Final = " << board.getFinalNodeString() << endl;
	cout << "Result = " << result << endl;
	cout << "Moves = " << board.getMoveListString() << endl;
	cout << "Count = " << board.getNodeCount() << endl;
	cout << "Elapsed = " << board.getElapsedTime() << endl;
	return 0;
}

int main() {
	execute16(B16::INITIAL_BP, B16::INITIAL_WP, 0, -(B16::CELLS+1), B16::CELLS+1);
	//14駒から
	execute36(551158016, 69329408, 0, -6, -2);
	return 0;
}
