use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_input(path: &Path) -> Vec<usize> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .map(|item| item.parse::<usize>())
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .collect();
    lines
}

fn main() {
    let path = Path::new("input/2021/day1.txt");
    let lines = read_input(path);

    println!("{:?}", lines);
}
