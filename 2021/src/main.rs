mod day1;

use std::fs;

fn main() {
  let input = fs::read_to_string("src/day1/input").expect("Something went wrong reading the file");
  let input_2 =
    fs::read_to_string("src/day1/input").expect("Something went wrong reading the file");
  println!("Day 1 result: {}", match day1::run(input) {
    Ok(v) => v.to_string(),
    Err(s) => s
  });
  println!("Day 1 result2: {}", match day1::run_2(input_2) {
    Ok(v) => v.to_string(),
    Err(s) => s
  });
  println!("");
}
