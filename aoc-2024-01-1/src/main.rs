use std::{
    collections::BinaryHeap,
    fmt::Display,
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let lists: Lists = lines
        .map(|line| line.unwrap().parse::<Pair>().unwrap())
        .collect();
    println!("sorted lists:");
    let result = lists.compute_difference();
    println!("{:?}", result);
}

#[derive(Debug)]
enum PairParseError {}

#[derive(Debug)]
struct Lists {
    first: BinaryHeap<usize>,
    second: BinaryHeap<usize>,
}

impl FromIterator<Pair> for Lists {
    fn from_iter<T: IntoIterator<Item = Pair>>(iter: T) -> Self {
        let mut lists = Lists {
            first: BinaryHeap::new(),
            second: BinaryHeap::new(),
        };

        for pair in iter {
            lists.first.push(pair.first);
            lists.second.push(pair.second);
        }
        lists
    }
}

impl Lists {
    fn compute_difference(self) -> usize {
        self.first
            .into_sorted_vec()
            .iter()
            .zip(self.second.into_sorted_vec().iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum()
    }
}

struct Pair {
    first: usize,
    second: usize,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.first, self.second)
    }
}

impl FromStr for Pair {
    type Err = PairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split_whitespace();
        let pair = Pair {
            first: i.next().unwrap().parse().unwrap(),
            second: i.next().unwrap().parse().unwrap(),
        };
        println!("{}", pair);
        Ok(pair)
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
