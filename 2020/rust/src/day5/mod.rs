use std::fs;

pub fn run(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  contents.split("\n").map(
    |encoded_num: &str| {
      encoded_num.to_owned().replace("F", "0").replace("B", "1").replace("R", "1").replace("L", "0")
    }
  ).map(|binary| {
    match usize::from_str_radix(&binary, 2) {
      Ok(u) => u,
      Err(error) => panic!("Unable to parse String({}), cause: {}", binary, error)
    }
  }).fold(0, |big, next| {
    if next > big {
      return next
    }
    big
  })
}

pub fn run_2(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  let mut nums = contents.split("\n").map(
    |encoded_num: &str| {
      encoded_num.to_owned().replace("F", "0").replace("B", "1").replace("R", "1").replace("L", "0")
    }
  ).map(|binary| {
    match usize::from_str_radix(&binary, 2) {
      Ok(u) => u,
      Err(error) => panic!("Unable to parse String({}), cause: {}", binary, error)
    }
  }).collect::<Vec::<usize>>();
  nums.sort();
  nums.into_iter().fold((0,0,false), |seat, next| {
    if seat.2 {
      return seat
    }
    if seat.1 == 0 || next == seat.1 + 1 {
      return (0, next, false)
    }
    (seat.1 + 1, next, true)
  }).0
}