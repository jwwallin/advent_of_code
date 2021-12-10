pub fn run(input: String) -> Result<i64, String> {
  let mut count = 0;
  let mut first = true;
  input
    .split('\n')
    .map(str::trim)
    .filter(|val| !val.is_empty())
    .flat_map(str::parse::<i64>)
    .fold(0, |prev, depth| {
      if !first {
        if depth > prev {
          count += 1;
        }
      } else {
        first = false;
      }
      depth
    });
  Ok(count)
}

pub fn run_2(input: String) -> Result<i64, String> {
  let vals: Vec<i64> = input
    .split('\n')
    .map(str::trim)
    .filter(|val| !val.is_empty())
    .flat_map(str::parse::<i64>)
    .collect();
  let mut count = 0;
  let mut prev = 0;
  // window is 3 long so stop before iterating beyond last index
  for i in 0..vals.len() - 2 {
    let window = vals
      .get(i..i + 3)
      .ok_or(String::from("Failed to get window."))?;
    let sum = window.iter().fold(0, |prev, v| prev + v);
    if i != 0 && sum > prev {
      count += 1;
    }
    prev = sum;
  }
  Ok(count)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() -> Result<(), String> {
    let input = "


    199
    200
    208
    210

    200
    207
    240

    269
    260

    263



";

    assert_eq!(7, run(String::from(input))?);
    Ok(())
  }
  #[test]
  fn test_run_2() -> Result<(), String> {
    let input = "


    199
    200
    208
    210

    200
    207
    240

    269
    260

    263



";

    assert_eq!(5, run_2(String::from(input))?);
    Ok(())
  }
}
