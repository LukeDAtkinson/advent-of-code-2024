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

    let err_ind = find_first_error_index(&parsed);
    match err_ind {
        None => {
            println!("Safe without removing any level");
            true
        }
        Some(i) => {
            let mut copy = parsed.to_vec();
            copy.remove(i);
            let copy_result = find_first_error_index(&copy);
            if copy_result == None {
                println!("Safe by removing {}", i);
                return true;
            }
            let mut copy = parsed.to_vec();
            copy.remove(i - 1);
            let copy_result = find_first_error_index(&copy);
            if copy_result == None {
                println!("Safe by removing index before {}", i);
                return true;
            }
            println!("Not safe");
            false
        }
    }
}

fn find_first_error_index(parsed: &Vec<isize>) -> Option<usize> {
    println!("Finding first error in {:?}", parsed);
    let first = parsed[0];
    let last = parsed[parsed.len()-1];
    let increasing = last > first;
    for i in 1..parsed.len() {
        let diff: isize = parsed[i] - parsed[i - 1];
        let relative_diff = if increasing { diff } else { -diff };
        if relative_diff < 1 || relative_diff > 3 {
            println!("First error is {}", i);
            return Some(i);
        }
    }
    println!("No error");
    None
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
