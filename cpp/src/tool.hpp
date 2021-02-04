#pragma once

#include <iostream>
#include <sstream>
#include <cassert>

using namespace std;

// exceptions
class OthelloException : virtual public exception {
	const string prefix{"OthelloException: "};
	string message;
public:
	OthelloException(const string& msg) {
		message = prefix + msg;
	}
	virtual ~OthelloException() throw() {}
	virtual const char* what() const throw () {
		return message.c_str();
	}
};

class Player {
public:
	//戻り値
	//不負=駒を置いた位置
	//-1=パス
	virtual int getMove(uint64_t inturn, uint64_t opponent) = 0;
};

template <class CTO>
struct Tool {
	constexpr static uint64_t ONE = 1;
	static string positiveToCell(int positive) {
		if (positive < 0) {
			// パス
			return "pa";
		}
		int value = positive % CTO::CELLS;
		int r = value % CTO::SIZE;
		int c = value / CTO::SIZE;
		char cell[3]{(char)(r + 0x61), (char)(c + 0x31), 0};
		return string(cell);
	}
	static int cellToPositive(string cell) {
		if (cell == "pa") {
			// パス
			return -1;
		}
		int c = (int)(cell[0] - 0x61);
		int r = (int)(cell[1] - 0x31);
		assert(c >= 0 && c < CTO::SIZE && r >= 0 && r < CTO::SIZE);
		return r * CTO::SIZE + c;
	}
	//
	// 盤上に駒を配置した文字列を戻す
	// bp:黑番, wp:白番
	//
	static string getBoardString(uint64_t bp, uint64_t wp) {
		ostringstream oss;
		uint64_t mask;
		oss << "  ";
		for (int c=0; c<CTO::SIZE; c++) {
			char code = 'a' + c;
			oss << code;
		}
		oss << endl;
		for (int i=0; i<CTO::CELLS; i++) {
			if (i % CTO::SIZE == 0) {
				int row = (int)(i / CTO::SIZE) + 1;
				oss << row << " ";
			}
			mask = ONE << i;
			if (bp & mask) {
				oss << 'X';
			} else if (wp & mask) {
				oss << 'O';
			} else {
				oss << '-';
			}
			if ((i + 1) % CTO::SIZE == 0) {
				oss << endl;
			}
		}
		return oss.str();
	}
	//
	// 盤上に駒を配置した文字列を戻す
	// 一変数版
	//
	static string getBoardString(uint64_t point) {
		ostringstream oss;
		uint64_t mask;
		oss << "  ";
		for (int c=0; c<CTO::SIZE; c++) {
			char code = 'a' + c;
			oss << code;
		}
		oss << endl;
		for (int i=0; i<CTO::CELLS; i++) {
			if (i % CTO::SIZE == 0) {
				int row = (int)(i / CTO::SIZE) + 1;
				oss << row << " ";
			}
			mask = ONE << i;
			if (point & mask) {
				oss << 'O';
			} else {
				oss << '-';
			}
			if ((i + 1) % CTO::SIZE == 0) {
				oss << endl;
			}
		}
		return oss.str();
	}
};
