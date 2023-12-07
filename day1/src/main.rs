use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(file_path: &str){
    println!("Day 1 Part 1: {}", get_calibration_value(file_path, false));
}

fn part2(file_path: &str) {
    println!("Day 1 Part 2: {}", get_calibration_value(file_path, true));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(file_path: &str, replace: bool) -> u32 {
    let mut result = 0;

    if let Ok(lines) = read_lines(file_path.to_string()) {
        for line in lines {
            if let Ok(s) = line {
            let formatted = get_formatted_line(&s, replace);
            let digits = formatted.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();

            result += digits.first().unwrap() * 10 + digits.last().unwrap()
            }
        }
    }

    return result;
}

fn get_formatted_line(line: &str, replace: bool) -> String {
    if replace {
        line.to_string()
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
    }
    else {
        line.to_string()
    }
}

fn main() {
    let input = "input.txt";
    part1(input);
    part2(input);
}
