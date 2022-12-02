use std::fs;

#[derive(Debug)]
struct BingoEntry {
    number: u32,
    marked: bool,
}

#[derive(Debug)]
struct BingoTable {
    width: u32,
    height: u32,
    data: Vec<BingoEntry>,
}

fn part_a(input: &str) {
    let file = fs::read_to_string(input).unwrap();
    let p: Vec<&str> = file.split("\\n\\n").collect();
    println!("{:?}", p);
    let numbers: Vec<u32> = file
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|c| u32::from_str_radix(c, 10).unwrap())
        .collect();
    // println!("{:?}", numbers);
}
fn main() {
    part_a("data/example");
}
