# othello
Compare performance between Rust and C++ with 6x6 othello

## Prequisites
+ C++ Compiler = g++ (~v8.3)
+ Rust = ~v1.49
+ Computer architecture = x86-64

## Execution
### C++
$ cd cpp
$ make
$ build/main.out
If you want to finish fast, try next
$ build/main.out 16
--------
[output]
Selected = 12
  abcdef
1 OOOOOX
2 XOOOXO
3 XXOXOO
4 XXXOXO
5 XOXXOO
6 XOXXOO

Initial = (bp, wp, turn) = (1753344, 81854976, 0), (alpha, beta) = (-6, -2)
Final = (bp, wp, turn) = (14182823008, 54536653727, 1), (alpha, beta) = (4, -2)
Result = -4
Moves = f5 b3 a3 c1 d5 e5 d1 e1 e2 a5 c6 a4 a6 e6 b5 b6 a2 b2 d6 f6 b1 a1 pa f2 f1
Count = 18282412125
Elapsed = 725.954
--------
If you want to finish fast, try
$ target/release/board 16

### Rust
$ cd rust/board
$ cargo rustc --release --bin board -- -C target-cpu=native
$ target/release/board
--------
[output]
Selected = 12
Result = -4
Initial = (bp, wp, turn) = (1753344, 81854976, 0), (alpha, beta) = (-6, -2)
Final = (bp, wp, turn) = (14182823008, 54536653727, 1), (alpha, beta) = (-4, -2)
Moves = f5 b3 a3 c1 d5 e5 d1 e1 e2 a5 c6 a4 a6 e6 b5 b6 a2 b2 d6 f6 b1 a1 pa f2 f1
Elapsed = 977.077587
--------
If you want to finish fast, try
$ target/release/board 16
