use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
fn main() {
    println!("{:?}", compute_result());
}

pub fn compute_result() -> usize {
    let lines = read_lines("input.txt").unwrap();
    let result: Result = lines.map(|line| line.unwrap()).collect();
    result.result
}

#[derive(Debug)]
struct Result {
    result: usize,
}

impl FromIterator<String> for Result {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let result = iter.into_iter().filter(|r| is_safe(r)).count();
        Result { result }
    }
}
fn is_safe(report: &String) -> bool {
    let parsed: Vec<isize> = report
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let first = parsed[0];
    let second = parsed[1];
    let increasing = second > first;
    for i in 1..parsed.len() {
        let diff: isize = parsed[i] - parsed[i - 1];
        let relative_diff = if increasing { diff } else { -diff };
        if relative_diff < 1 || relative_diff > 3 {
            return false;
        }
    }
    true
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
