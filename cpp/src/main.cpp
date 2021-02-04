#include <iostream>
#include "b36.hpp"
#include "board.hpp"

using namespace std;

void execute(uint64_t bp, uint64_t wp, int turn, int alpha, int beta) {
	Board<B36> board;
	int result = board.getBestResult(bp, wp, turn, alpha, beta);

	cout << board.getFinalBoardString() << endl;
	cout << "Initial = " << board.getInitialNodeString() << endl;
	cout << "Final = " << board.getFinalNodeString() << endl;
	cout << "Result = " << result << endl;
	cout << "Moves = " << board.getMoveListString() << endl;
	cout << "Count = " << board.getNodeCount() << endl;
	cout << "Elapsed = " << board.getElapsedTime() << endl;
}

//12駒から
int main12() {
	execute(1753344, 81854976, 0, -6, -2);
	return 0;
}

//14駒から
int main14() {
	execute(551158016, 69329408, 0, -6, -2);
	return 0;
}

//16駒から
int main16() {
	execute(550219776, 70271748, 0, -6, -2);
	return 0;
}

int main(int narg, char** argv) {
	int sel = 12;
	if (narg > 1) {
		sel = atoi(argv[1]);
	}
	cout << "Selected = " << sel << endl;
	switch(sel) {
		case 14:	return main14();
		case 16:	return main16();
		default:	return main12();
	}
}
