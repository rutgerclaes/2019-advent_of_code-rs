use std::fs;
use std::io::Error;

pub fn load_file(path: &str) -> Result<String, Error> {
  fs::read_to_string(path)
}

pub fn read_integers(vec: &Vec<&str>) -> Vec<i64> {
  vec
    .iter()
    .map(|line| line.parse::<i64>().unwrap())
    .collect()
}
