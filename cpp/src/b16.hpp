struct B16 {
	constexpr static int SIZE = 4;
	constexpr static int CELLS = SIZE * SIZE;
	constexpr static uint64_t MASK = 0x000000000000FFFF;
	constexpr static uint64_t INITIAL_BP = 576;
	constexpr static uint64_t INITIAL_WP = 1056;
	constexpr static int INITIAL_MOVE = 11;
	constexpr static uint64_t RV_MASK0 = 0x0000000000006666;
	constexpr static uint64_t RV_MASK1 = 0x0000000000000FF0;
	constexpr static uint64_t RV_MASK2 = 0x0000000000000660;

	static uint64_t getUpward(int inc, uint64_t mask, uint64_t inturn, uint64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t t = w & (inturn << inc);
		t |= (w & (t << inc));
		t |= (w & (t << inc));
	  return (t << inc);
	}

	static uint64_t getDownward(int inc, uint64_t mask, uint64_t inturn, uint64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t t = w & (inturn >> inc);
		t |= (w & (t >> inc));
		t |= (w & (t >> inc));
	  return (t >> inc);
	}

	static uint64_t reverseUpward(int inc, uint64_t mask, uint64_t tmask, int64_t inturn, int64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t e1 = (tmask << inc) & w;
	  uint64_t b1 = (inturn >> inc) & w;
	  uint64_t rev = (e1 << inc) & b1 & w;
	  uint64_t b2 = (b1 >> inc) & w;
	  return rev | (e1 & (b1 | b2));
	}

	static uint64_t reverseDownward(int inc, uint64_t mask, uint64_t tmask, int64_t inturn, int64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t e1 = (tmask >> inc) & w;
	  uint64_t b1 = (inturn << inc) & w;
	  uint64_t rev = (e1 >> inc) & b1 & w;
	  uint64_t b2 = (b1 << inc) & w;
	  return rev | (e1 & (b1 | b2));
	}
};
