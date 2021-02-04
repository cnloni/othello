use board::board::Board;
use bitop::b36::B36;
use std::env;

fn execute(bp : u64, wp : u64, turn : i32, alpha : i32, beta : i32) {
  let mut board : Board<B36> = Board::<B36>::new();
	let result : i32 = board.get_best_result_with_ab(bp, wp, turn, alpha, beta);
  println!("Result = {}", result);
  println!("Initial = {}", board.get_initial());
  println!("Final = {}", board.get_final());
  println!("Moves = {}", board.get_move_list_string());
  println!("Elapsed = {}", board.get_elapsed());
}

//12駒から
fn main12() {
	execute(1753344, 81854976, 0, -6, -2);
}

//14駒から
fn main14() {
	execute(551158016, 69329408, 0, -6, -2);
}

//16駒から
fn main16() {
	execute(550219776, 70271748, 0, -6, -2);
}

fn main() -> Result<(), ()> {
  let args: Vec<String> = env::args().collect();
  let mut sel : i32 = 12;
  if args.len() > 1 {
    sel = args[1].parse().unwrap();
  }
  println!("Selected = {}", sel);
  match sel {
    14 => main14(),
    16 => main16(),
    _  => main12()
  }
  Ok(())
}
