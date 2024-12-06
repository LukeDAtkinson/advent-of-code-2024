use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let lists: Counts = lines.map(|line| line.unwrap()).collect();
    let result = lists.compute_similarity();
    println!("{:?}", result);
}

#[derive(Debug)]
struct Counts {
    counts: HashMap<usize, usize>,
}

impl FromIterator<String> for Counts {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut counts = Counts {
            counts: HashMap::new(),
        };

        let mut second: Vec<usize> = Vec::new();
        for s in iter {
            let mut i = s.split_whitespace();
            counts.counts.insert(i.next().unwrap().parse().unwrap(), 0);
            second.push(i.next().unwrap().parse().unwrap());
        }
        for it in second {
            counts.counts.entry(it).and_modify(|e| *e += 1);
        }
        counts
    }
}

impl Counts {
    fn compute_similarity(&self) -> usize {
        self.counts.iter().map(|(k,v)| {
            k*v
        }).sum()
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
