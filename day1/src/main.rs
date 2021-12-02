use std::fs;

fn main() {
    let filename = "data/input";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let measurements: Vec<u32> = contents
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut counter = 0;
    let mut last_elem: Option<u32> = None;
    for m in measurements {
        if last_elem.is_some() {
            if m > last_elem.unwrap() {
                counter += 1;
            }
        }
        last_elem = Some(m);
    }
    println!("counter: {}", counter)
}
