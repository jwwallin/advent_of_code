use std::fs;

pub fn run(input: &str) -> i64 {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  let lines = contents.split("\n");

  let mut valid = 0;

  for line in lines {
    if line.is_empty() {
      continue;
    }
    let line = String::from(line);
    let mut parts = line.split(":");
    let policy = String::from(parts.next().unwrap().trim());
    let mut policy = policy.split(" ");
    let password = String::from(parts.next().unwrap().trim());
    let policy_limit = String::from(policy.next().unwrap().trim());
    let mut policy_limit =  policy_limit.split("-");
    let policy_letter = policy.next().unwrap();
    let min = policy_limit.next().unwrap().parse::<usize>().unwrap();
    let max = policy_limit.next().unwrap().parse::<usize>().unwrap();

    let count = password.split(policy_letter).count() - 1;
    if count >= min && count <= max {
      valid += 1;
    }
  }
  valid
}

pub fn run_2(input: &str) -> i64 {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  let lines = contents.split("\n");

  let mut valid = 0;

  for line in lines {
    if line.is_empty() {
      continue;
    }
    let line = String::from(line);
    let mut parts = line.split(":");
    let policy = String::from(parts.next().unwrap().trim());
    let mut policy = policy.split(" ");
    let password = String::from(parts.next().unwrap().trim());
    let policy_limit = String::from(policy.next().unwrap().trim());
    let mut policy_limit =  policy_limit.split("-");
    let policy_letter = String::from(policy.next().unwrap());
    let policy_letter = policy_letter.trim().chars().nth(0).unwrap();
    let pos1 = policy_limit.next().unwrap().parse::<usize>().unwrap() - 1;
    let pos2 = policy_limit.next().unwrap().parse::<usize>().unwrap() - 1;

    let char1 = match password.chars().nth(pos1) {
      Some(n) => n,
      None => continue
    };
    let char2 = match password.chars().nth(pos2) {
      Some(n) => n,
      None => continue
    };
    if (char1 == policy_letter && char2 == policy_letter) 
      || (char1 != policy_letter && char2 != policy_letter) {
      continue;
    }
    valid += 1;
  }
  valid
}
