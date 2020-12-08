use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  let bag_map = parse(contents);
  let can_contain_shiny_gold: HashSet<String> = can_contain(&"shiny gold".to_owned(), &bag_map);
  can_contain_shiny_gold.len()
}

pub fn run_2(input: &str) -> usize {
  let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
  let bag_map = parse(contents);
  
  0
}

fn can_contain(target_bag: &String, bag_map: &HashMap<String, HashMap<String, usize>>) -> HashSet<String> {
  let mut set = HashSet::new();
  for bag in bag_map.keys() {
    let content: &HashMap<String, usize> = bag_map.get(bag).unwrap();
    if content.contains_key(target_bag) {
      set.insert(bag.to_owned());
    }
  }
  if set.is_empty() {
    return set
  }
  let clone: Vec<String> = set.iter().cloned().collect();
  for bag in clone {
    for b in can_contain(&bag, bag_map) {
      set.insert(b);
    }
  }
  set
}

fn parse(rules: String) -> HashMap<String, HashMap<String, usize>> {
  let mut map = HashMap::new();
  let rules = rules.split("\n");
  for rule in rules {
    let rule = rule.to_owned();
    let mut split = rule.split(" bags contain ");
    let bag = match split.next() {
      Some(s) => s.to_owned(),
      None => panic!()
    };
    let content = match split.next() {
      Some(s) => s.to_owned(),
      None => panic!()
    };
    let mut bags = HashMap::new();
    if content.contains("no other bags") {
    } else {
      for bag in content.split(",") {
        let bag = bag.trim().to_owned();
        let bag = bag.trim_end_matches("bags.")
                     .trim_end_matches("bag.")
                     .trim_end_matches("bags")
                     .trim_end_matches("bag")
                     .trim()
                     .to_owned();
        let num = bag.chars().nth(0).unwrap().to_string();
        let num = num.parse::<usize>().unwrap();
        let name = (&bag[2..bag.len()]).to_owned();
        bags.insert(name, num);
      }
    }
    map.insert(bag, bags);
  }
  map
}

