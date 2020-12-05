use std::fs;

pub fn run(input: &str) -> u128 {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  let grid = get_grid(contents);

  get_trees_encountered(&grid, (3, 1))
}

pub fn run_2(input: &str) -> u128 {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  let grid = get_grid(contents);

  let mut slopes = Vec::new();

  slopes.push((1, 1));
  slopes.push((3, 1));
  slopes.push((5, 1));
  slopes.push((7, 1));
  slopes.push((1, 2));

  slopes.into_iter().fold(1, |product, slope| {
    product * get_trees_encountered(&grid, slope)
  })
}

fn get_trees_encountered(grid: &Grid, slope: (usize, usize)) -> u128 {

  let mut x = 0;
  let mut y = 0;
  let height = grid.height();

  let mut tree_count = 0;

  loop {
    x += slope.0;
    y += slope.1;

    if y >= height {
      break;
    }

    let node = grid.get(x, y);

    if node == '#' {
      tree_count += 1;
    }

  }

  tree_count
}

fn get_grid(grid_string: String) -> Grid {
  let mut grid: Vec<Vec<char>> = Vec::new();

  let lines = grid_string.split("\n");
  
  for line in lines {
    let line = String::from(line);
    let chars = line.chars().collect::<Vec<char>>();
    grid.push(chars);
  }

  Grid { lines: grid }
}

struct Grid {
  lines: Vec<Vec<char>>
}

impl Grid {
  pub fn height(&self) -> usize {
    self.lines.len()
  }

  fn width(&self) -> usize {
    self.lines.get(0).unwrap().len()
  }

  pub fn get(&self, x: usize, y: usize) -> char {
    self.lines.get(y).unwrap()
      .get(x % self.width()).unwrap()
      .clone()
  }
}
