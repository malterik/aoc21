use std::fs;

enum Command {
    Forward(u16),
    Down(u16),
    Up(u16)
}

fn parse_commands(file: &str) {
    let file_content = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    let commands_str = file_content.lines();
    // let commands = commands_str.iter()
    //     .map( |command| {
    //     })
    //     .collect();
    for command in commands_str {
        let direction = command.split(" ").collect::<Vec<&str>>()[0];
        let value = command.split(" ").collect::<Vec<&str>>()[1];
        println!("dir: {}, val: {}", direction, value);
    }

            
}

fn main() {
    let command = parse_commands("data/example");
}
