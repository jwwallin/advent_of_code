pub fn run(input: String) -> Result<i64, String> {
    let mut forward = 0;
    let mut depth = 0;

    input
        .split('\n')
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(String::from)
        .flat_map(Move::try_from)
        .try_for_each(|mov| match mov.dir.as_str() {
            "forward" => {
                forward += mov.dist;
                Ok(())
            }
            "up" => {
                depth -= mov.dist;
                Ok(())
            }
            "down" => {
                depth += mov.dist;
                Ok(())
            }
            _ => Err(String::from(format!("Unexpected direction {}", mov.dir))),
        })?;
    Ok(forward * depth)
}

pub fn run_2(input: String) -> Result<i64, String> {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .split('\n')
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(String::from)
        .flat_map(Move::try_from)
        .try_for_each(|mov| match mov.dir.as_str() {
            "forward" => {
                forward += mov.dist;
                depth += aim * mov.dist;
                Ok(())
            }
            "up" => {
                aim -= mov.dist;
                Ok(())
            }
            "down" => {
                aim += mov.dist;
                Ok(())
            }
            _ => Err(String::from(format!("Unexpected direction {}", mov.dir))),
        })?;
    Ok(forward * depth)
}

struct Move {
    dir: String,
    dist: i64,
}

impl TryFrom<String> for Move {
    type Error = String;

    fn try_from(s: String) -> Result<Move, Self::Error> {
        let split: Vec<&str> = s.split(' ').collect();

        let dir = split.get(0).ok_or(String::from("no direction in string"))?;
        let dist = split.get(1).ok_or(String::from("no distance in string"))?;
        let dist = dist
            .parse::<i64>()
            .map_err(|_| String::from("failed to parse distance"))?;

        Ok(Move {
            dir: String::from(*dir),
            dist: dist,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> Result<(), String> {
        let input = "

    forward 5

    down 5
    forward 8

    up 3
    down 8
    forward 2

    ";
        assert_eq!(150, run(String::from(input))?);
        Ok(())
    }
    #[test]
    fn test_run_2() -> Result<(), String> {
        let input = "

    forward 5

    down 5
    forward 8

    up 3
    down 8
    forward 2

    ";
        assert_eq!(900, run_2(String::from(input))?);
        Ok(())
    }
}
