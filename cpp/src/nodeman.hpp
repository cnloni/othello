#include <cassert>
#include <cstring>
#include <vector>

template <class CTO>
class NodeManager {
public:
	static constexpr int BLANK_ID = 0x10000;
	static constexpr int PASS_BASE = 1000;
private:
	int nextId{0};
	int* mem{nullptr};
	int size{0};
public:
	NodeManager() {
	}
	virtual ~NodeManager() {
		free();
	}
	void allocate(int n) {
		free();
		size = (n + 2) * (n + 1) / 2;
		mem = new int[size];
	}
	void free() {
		if (mem != nullptr) {
			delete []mem;
			mem = nullptr;
		}
	}
	void clear(uint64_t bp, uint64_t wp) {
		nextId = 0;
		//盤上の空き数
		int blanks = CTO::CELLS - bitcount(bp | wp);
		allocate(blanks);
	}
	int setMove(int move, int pass) {
		assert(nextId < size && move >= 0 && move < CTO::CELLS && pass >= 0 && pass <= 2);
		mem[nextId++] = move + (pass * PASS_BASE);
		return nextId - 1;
	}
	void unsetMoves(int from) {
		nextId = from;
	}
	//(idOver, nextId - 1)をidUnder以下に上書きする
	int transoverMoves(int idOver, int idUnder) {
			if (idUnder == BLANK_ID) {
				//上書き部分が存在しないので何もしない
				return idOver;
			} else {
				size_t msize = (nextId - idOver) * sizeof(int);
				memmove(mem + idUnder, mem + idOver, msize);
				nextId -= (idOver - idUnder);
				return idUnder;
			}
	}
	vector<int> getList() {
		vector<int> list;
		for(int i=0; i<nextId; i++) {
			int pass = mem[i] / PASS_BASE;
			int move = mem[i] % PASS_BASE;
			if (pass == 2) {
				list.push_back(-1);
				list.push_back(-1);
			} else {
				if (pass == 1) {
					list.push_back(-1);
				}
				list.push_back(move);
			}
		}
		return list;
	}
};
