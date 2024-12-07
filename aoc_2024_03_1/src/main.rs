use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file contents to string");
    let result: usize = Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap())
        .sum();

    println!("{}", result);
}
