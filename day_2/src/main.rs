extern crate day_2;
extern crate utils;

use std::env;

/// # The Tyranny of the Rocket Equation
fn main() {
    let input_file = env::args()
        .skip(1)
        .next()
        .expect("Pass the input file as first parameter");

    println!("==== [AOC] Day 2 ====");
    println!("Reading data from {}", input_file);

    let data = utils::load_file(&input_file).expect("Couldn't read input file");
    let numbers: Vec<i32> = data
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("--- Part 1 ---");
    println!("Result: {}", day_2::part_1(&numbers));

    println!("--- Part 2 ---");
    println!("Result: {}", day_2::part_2(&numbers));
}
