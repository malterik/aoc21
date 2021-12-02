use std::fs;

fn main() {
    let filename = "data/example";

    let measurements: Vec<u32> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let windows = measurements.windows(3);
    for window in windows {
        println!("window:\n{:?}", window.iter().sum::<u32>());
    }
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
