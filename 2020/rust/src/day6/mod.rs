use std::fs;

pub fn run(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  contents.split("\n\n").fold(0, |total, next| {
    total + next.to_owned().chars().fold((0, String::new()), |mut cumul, c| {
      if c == '\n' {
        return cumul
      }
      if cumul.1.contains(c) {
        return cumul
      }
      cumul.1.push(c);
      (cumul.0 + 1, cumul.1)
    }).0
  })
}

pub fn run_2(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  contents.split("\n\n").fold(0, |total, next| {
    let mut next = next.to_owned();
    next.push('\n');
    let chars = next.chars();
    let mut first_person = true;
    let mut everyone_answered = String::new();
    let mut answered = String::new();
    for c in chars {
      if c == '\n' {
        if first_person {
          everyone_answered = answered;
          first_person = false;
        } else {
          everyone_answered.retain(|c| {
            answered.contains(c)
          })
        }
        answered = String::new();
      } else {
        answered.push(c);
      }
    }
    total + everyone_answered.len()
  })
}