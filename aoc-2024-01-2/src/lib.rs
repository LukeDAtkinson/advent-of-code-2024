use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn compute_result() -> usize {
    let lines = read_lines("input.txt").unwrap();
    let result: Result = lines.map(|line| line.unwrap()).collect();
    result.result

}

#[derive(Debug)]
struct Result {
    result: usize
}

impl FromIterator<String> for Result {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {

        let mut first: HashSet<usize> = HashSet::new();
        let mut second: Vec<usize> = Vec::new();
        for s in iter {
            let mut i = s.split_whitespace();
            first.insert(i.next().unwrap().parse().unwrap());
            second.push(i.next().unwrap().parse().unwrap());
        }
        let result = second.iter().filter(|it|first.contains(it)).sum();

        Result{ result }
    }
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
