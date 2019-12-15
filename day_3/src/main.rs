extern crate day_3;
extern crate utils;

use std::env;

/// # Crossed Wires
fn main() {
    let input_file = env::args()
        .skip(1)
        .next()
        .expect("Pass the input file as first parameter");

    println!("==== [AOC] Day 3 ====");
    println!("Reading data from {}", input_file);

    let data = utils::load_file(&input_file).expect("Couldn't read input file");
    let lines: Vec<&str> = data.lines().collect();

    println!("--- Part 1 ---");
    println!("Result: {}", day_3::part_1(&lines));

    println!("--- Part 2 ---");
    println!("Result: {}", day_3::part_2(&lines));
}
