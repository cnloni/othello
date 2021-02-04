struct B36 {
	constexpr static int SIZE = 6;
	constexpr static int CELLS = SIZE * SIZE;
	constexpr static uint64_t MASK = 0x0000000FFFFFFFFF;
	constexpr static uint64_t INITIAL_BP = 1081344;
	constexpr static uint64_t INITIAL_WP = 2113536;
	constexpr static int INITIAL_MOVE = 22;
	constexpr static uint64_t RV_MASK0 = 0x000000079E79E79E;
	constexpr static uint64_t RV_MASK1 = 0x000000003FFFFFC0;
	constexpr static uint64_t RV_MASK2 = 0x000000001E79E780;

	static uint64_t getUpward(int inc, uint64_t mask, uint64_t inturn, uint64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t t = w & (inturn << inc);
		t |= (w & (t << inc));
		t |= (w & (t << inc));
		t |= (w & (t << inc));
		t |= (w & (t << inc));
	  return (t << inc);
	}

	static uint64_t getDownward(int inc, uint64_t mask, uint64_t inturn, uint64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t t = w & (inturn >> inc);
		t |= (w & (t >> inc));
		t |= (w & (t >> inc));
		t |= (w & (t >> inc));
		t |= (w & (t >> inc));
	  return (t >> inc);
	}

	static uint64_t reverseUpward(int inc, uint64_t mask, uint64_t tmask, int64_t inturn, int64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t e1 = (tmask << inc) & w;
	  uint64_t e2 = (e1 << inc) & w;
	  uint64_t e3 = (e2 << inc) & w;
	  uint64_t e4 = (e3 << inc) & w;
	  uint64_t b1 = (inturn >> inc) & w;
	  uint64_t rev = e4 & b1;
	  uint64_t b2 = (b1 >> inc) & w;
	  rev |= e3 & (b1 |= b2);
	  b2 = (b2 >> inc) & w;
	  rev |= e2 & (b1 |= b2);
	  b2 = (b2 >> inc) & w;
	  return rev | (e1 & (b1 | b2));
	}

	static uint64_t reverseDownward(int inc, uint64_t mask, uint64_t tmask, int64_t inturn, int64_t opponent) {
	  uint64_t w = opponent & mask;
	  uint64_t e1 = (tmask >> inc) & w;
	  uint64_t e2 = (e1 >> inc) & w;
	  uint64_t e3 = (e2 >> inc) & w;
	  uint64_t e4 = (e3 >> inc) & w;
	  uint64_t b1 = (inturn << inc) & w;
	  uint64_t rev = e4 & b1;
	  uint64_t b2 = (b1 << inc) & w;
	  rev |= e3 & (b1 |= b2);
	  b2 = (b2 << inc) & w;
	  rev |= e2 & (b1 |= b2);
	  b2 = (b2 << inc) & w;
	  return rev | (e1 & (b1 | b2));
	}
};
