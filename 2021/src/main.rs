mod day1;
mod day2;

use std::fs;

fn main() {
  let input = fs::read_to_string("src/day2/input").expect("Something went wrong reading the file");
  let input_2 =
    fs::read_to_string("src/day2/input").expect("Something went wrong reading the file");
  println!("Result: {}", match day2::run(input) {
    Ok(v) => v.to_string(),
    Err(s) => s
  });
  println!("Result2: {}", match day2::run_2(input_2) {
    Ok(v) => v.to_string(),
    Err(s) => s
  });
  println!();
}
