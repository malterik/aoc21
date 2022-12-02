use std::fs;
use transpose;

struct Report {
    gamma_rate: u32,
    epsilon_rate: u32,
    report: Vec<u32>,
}

impl Report {
    fn new(file: &str) -> Self {
        let report: Vec<u32> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect();

        Report {
            gamma_rate: 0,
            epsilon_rate: 0,
            report,
        }
    }

    fn get_power_consumption(&mut self, number_of_bits: u32) -> u32 {
        self.get_epsilon_rate(number_of_bits);
        self.get_gamma_rate(number_of_bits);
        self.epsilon_rate * self.gamma_rate
    }

    fn create_matrix(&self, input: &Vec<u32>, number_of_bits: u32) -> Vec<u32> {
        let mut result = Vec::<u32>::new();
        let a = |v, i| {
            if (v & (1 << i) as u32) == 0 {
                0
            } else {
                1
            }
        };
        for number in input {
            for i in (0..number_of_bits).rev() {
                result.push(a(number, i));
            }
        }
        result
    }

    fn get_column_vectors(&self, input: &Vec<u32>, number_of_bits: u32) -> Vec<u32> {
        let mut output = vec![0; 12000];
        transpose::transpose(
            &input,
            &mut output,
            number_of_bits.try_into().unwrap(),
            self.report.len(),
        );
        return output;
    }

    fn get_gamma_rate(&mut self, number_of_bits: u32) -> u32 {
        let matrix = self.create_matrix(&self.report, number_of_bits);
        let col_vec = self.get_column_vectors(&matrix, number_of_bits);
        let chunks: Vec<&[u32]> = col_vec.chunks(self.report.len()).collect();
        for (i, x) in chunks.iter().enumerate() {
            let index = (number_of_bits - 1) as usize - i;
            let number_of_ones = x.iter().filter(|&n| *n == 1).count();
            let number_of_zeroes = x.iter().filter(|&n| *n == 0).count();
            // println!("chunk[{}] = {:?}, ones: {}, zeroes: {} ", index, x, number_of_ones, number_of_zeroes);
            if number_of_ones > number_of_zeroes {
                self.gamma_rate = self.gamma_rate | (1 << index);
            }
        }
        println!("gamma: {}", self.gamma_rate);
        self.gamma_rate
    }

    fn get_epsilon_rate(&mut self, number_of_bits: u32) -> u32 {
        let matrix = self.create_matrix(&self.report, number_of_bits);
        let col_vec = self.get_column_vectors(&matrix, number_of_bits);
        let chunks: Vec<&[u32]> = col_vec.chunks(self.report.len()).collect();
        for (i, x) in chunks.iter().enumerate() {
            let index = (number_of_bits - 1) as usize - i;
            let number_of_ones = x.iter().filter(|&n| *n == 1).count();
            let number_of_zeroes = x.iter().filter(|&n| *n == 0).count();
            // println!("chunk[{}] = {:?}, ones: {}, zeroes: {} ", index, x, number_of_ones, number_of_zeroes);
            if number_of_ones < number_of_zeroes {
                self.epsilon_rate = self.epsilon_rate | (1 << index);
            }
        }
        println!("epsilon_rate: {}", self.epsilon_rate);
        self.epsilon_rate
    }
}

fn main() {
    let mut diag_report = Report::new("data/input");
    println!(
        "energy consumption: {}",
        diag_report.get_power_consumption(12)
    );
    part2("data/input");
}

fn part2(input: &str) {
    println!("----------Part2-------------");
    let mut file = fs::read_to_string(input).unwrap();
    let mut report: Vec<&str> = file.lines().collect();
    let mut report_co2 = report.clone();
    let num_of_bits = report[0].len();
    for i in 0..num_of_bits {
        let mut zero_counter = 0;
        let mut one_counter = 0;
        for line in &report {
            match line.chars().nth(i).unwrap() {
                '1' => one_counter += 1,
                '0' => zero_counter += 1,
                _ => panic!("Error in report"),
            };
        }
        if one_counter >= zero_counter {
            report = report.clone().into_iter().filter( |l| l.chars().nth(i).unwrap() == '1').collect();
        } else  {
            report = report.clone().into_iter().filter( |l| l.chars().nth(i).unwrap() == '0').collect();
        }
        if report_co2.len() == 1 {
            break;
        }
        println!("report:\n{:?}", report);
    }
    let oxygen_rating = u32::from_str_radix(report[0], 2).unwrap();
    println!("oxygen_rating: {}",  oxygen_rating);

    for i in 0..num_of_bits {
        let mut zero_counter = 0;
        let mut one_counter = 0;
        for line in &report_co2 {
            match line.chars().nth(i).unwrap() {
                '1' => one_counter += 1,
                '0' => zero_counter += 1,
                _ => panic!("Error in report"),
            };
        }
        if zero_counter <= one_counter {
            report_co2 = report_co2.clone().into_iter().filter( |l| l.chars().nth(i).unwrap() == '0').collect();
        } else {
            report_co2 = report_co2.clone().into_iter().filter( |l| l.chars().nth(i).unwrap() == '1').collect();
        }
        if report_co2.len() == 1 {
            break;
        }
    }
    let scrubber_rating = u32::from_str_radix(report_co2[0], 2).unwrap();
    println!("scrubber_rating: {}",  scrubber_rating);
    println!("life support rating: {}", scrubber_rating * oxygen_rating);
}

#[cfg(test)]
mod test_submarine {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let diag_report = Report::new("data/example");
        let input = vec![4, 30, 22];
        let expected_result = vec![0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0];
        let result = diag_report.create_matrix(&input, 5);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_gamma() {
        let mut diag_report = Report::new("data/example");
        assert_eq!(diag_report.get_gamma_rate(5), 0b10110);
    }

    #[test]
    fn test_epsilon() {
        let mut diag_report = Report::new("data/example");
        assert_eq!(diag_report.get_epsilon_rate(5), 0b01001);
    }
    #[test]
    fn test_part1() {
        let mut diag_report = Report::new("data/example");
        assert_eq!(diag_report.get_power_consumption(5), 198);
    }
    // #[test]
    // fn test_part2() {
    //     unimplemented!()
    // }
}
