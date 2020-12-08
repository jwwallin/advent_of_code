mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!("Hello, world!");

    println!("Day 1 result: {}", day1::run("src/day1/input"));
    println!("Day 1 result2: {}", day1::run_2("src/day1/input"));
    println!("");

    println!("Day 2 result: {}", day2::run("src/day2/input"));
    println!("Day 2 result2: {}", day2::run_2("src/day2/input"));
    println!("");

    println!("Day 3 result: {}", day3::run("src/day3/input"));
    println!("Day 3 result2: {}", day3::run_2("src/day3/input"));
    println!("");

    println!("Day 4 result: {}", day4::run("src/day4/input"));
    println!("Day 4 result2: {}", day4::run_2("src/day4/input"));
    println!("");

    println!("Day 5 result: {}", day5::run("src/day5/input"));
    println!("Day 5 result2: {}", day5::run_2("src/day5/input"));
    println!("");

    println!("Day 6 result: {}", day6::run("src/day6/input"));
    println!("Day 6 result2: {}", day6::run_2("src/day6/input"));
    println!("");

    println!("Day 7 result: {}", day7::run("src/day7/input"));
    println!("Day 7 result2: {}", day7::run_2("src/day7/test_input"));
    println!("");
}
