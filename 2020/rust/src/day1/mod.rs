use std::fs;

pub fn run() -> i64 {
  let contents = fs::read_to_string("src/day1/input")
        .expect("Something went wrong reading the file");
  let numbers: Vec<&str> = contents.split_whitespace().collect();

  let mut index = 0;

  let mut num1 = 0;
  let mut num2 = 0;

  while index < numbers.len() {
    num1 = numbers.get(index).unwrap().parse::<i64>().unwrap();
    let mut index2 = index + 1;
    while index2 < numbers.len() {
      num2 = numbers.get(index2).unwrap().parse::<i64>().unwrap();

      if num1 + num2 == 2020 {
        break;
      }
      
      index2 += 1;
    }

    if num1 + num2 == 2020 {
      break;
    }
    index += 1;
  }
  num1 * num2
}

pub fn run_2() -> i64 {
  let contents = fs::read_to_string("src/day1/input")
        .expect("Something went wrong reading the file");
  let numbers: Vec<&str> = contents.split_whitespace().collect();

  let mut index = 0;

  let mut num1 = 0;
  let mut num2 = 0;
  let mut num3 = 0;

  while index < numbers.len() {
    num1 = numbers.get(index).unwrap().parse::<i64>().unwrap();
    let mut index2 = index + 1;
    while index2 < numbers.len() {
      num2 = numbers.get(index2).unwrap().parse::<i64>().unwrap();
      
      let mut index3 = index2 + 1;

      while index3 < numbers.len() {
        num3 = numbers.get(index3).unwrap().parse::<i64>().unwrap();
  
        if num1 + num2 + num3 == 2020 {
          break;
        }
        
        index3 += 1;
      }

      if num1 + num2 + num3 == 2020 {
        break;
      }
      
      index2 += 1;
    }

    if num1 + num2 + num3 == 2020 {
      break;
    }
    index += 1;
  }
  num1 * num2 * num3
}
