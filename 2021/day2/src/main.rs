use std::fs;
use std::fmt;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32)
}

#[derive(Debug)]
enum Part {
    One,
    Two
}

#[derive(Debug)]
struct CommandList {
    commands: Vec<Command>,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Forward(v) => write!(f, "Forward {}", v),
            Command::Down(v) => write!(f, "Down {}", v),
            Command::Up(v) => write!(f, "Up {}", v)
        }
    }
}

impl CommandList {
    fn from_file(file: &str) -> CommandList {
        let file_content = fs::read_to_string(file)
            .expect("Something went wrong reading the file");
        let commands_str = file_content.lines();
        let mut command_vec: Vec<Command> = Vec::new();
        for command in commands_str {
            let direction = command.split(' ').collect::<Vec<&str>>()[0];
            let value: u32 = command.split(' ').collect::<Vec<&str>>()[1].parse().unwrap();
            match direction {
                "forward" => command_vec.push(Command::Forward(value)),
                "down" => command_vec.push(Command::Down(value)),
                "up" => command_vec.push(Command::Up(value)),
                _ => println!("Unknown Command"),
            }
        }
        CommandList {
            commands: command_vec,
        }
    }
}

#[derive(Debug)]
struct Submarine {
    command_list: CommandList,
    depth: u32,
    position: u32,
    aim: u32,
}

impl Submarine {
    fn new(command_list: CommandList) -> Self {
        Submarine {
            command_list,
            depth: 0,
            position: 0,
            aim: 0,
        }
    }

    fn drive(&mut self, part: Part) {
        match part {
            Part::One => {
                for c in &self.command_list.commands {
                    match c {
                        Command::Forward(v) => self.position += v,
                        Command::Down(v) => self.depth += v,
                        Command::Up(v) => self.depth -= v,
                    };
                }
            }
            Part::Two => {
                for c in &self.command_list.commands {
                    match c {
                        Command::Forward(v) => {
                            self.position += v;
                            self.depth += self.aim * v;
                        }
                        Command::Down(v) => self.aim += v,
                        Command::Up(v) => self.aim -= v,
                    };
                }
            }
        }
    }

    fn print_position(&self) {
        println!("depth: {}\nposition: {}", self.depth, self.position);
    }

    fn get_position(&self) -> (u32, u32) {
        (self.position, self.depth)
    }

    fn get_result(&self) -> u32 {
        self.depth * self.position
    }

    fn print_result(&self) {
        println!("result: {}", self.depth * self.position);
    }
}


fn main() {
    let commands = CommandList::from_file("data/input");
    let mut sub = Submarine::new(commands);
    sub.drive(Part::Two);
    sub.print_position();
    sub.print_result();
}

#[cfg(test)]
mod test_submarine {
    use super::*;

    #[test]
    fn test_part1() {
        let commands = CommandList::from_file("data/example");
        let mut sub = Submarine::new(commands);
        sub.drive(Part::One);
        assert_eq!(sub.get_position(), (15, 10));
        assert_eq!(sub.get_result(), 150);
    }
    #[test]
    fn test_part2() {
        let commands = CommandList::from_file("data/example");
        let mut sub = Submarine::new(commands);
        sub.drive(Part::Two);
        assert_eq!(sub.get_position(), (15, 60));
        assert_eq!(sub.get_result(), 900);
    }
}
