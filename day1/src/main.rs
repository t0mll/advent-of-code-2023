use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut n = 0;
    if let Ok(lines) = read_lines("input.txt"){
        for line in lines {
            if let Ok(s) = line {
                n += get_calibration_value(&s)   
            }
        }
    }

    println!("{}", n);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(s: &str) -> u32 {
    let mut left = u32::MIN;
    let mut right = u32::MIN;

     for ch in s.chars() {
        if ch.is_digit(10) {
            if left == u32::MIN {
                    left = ch.to_digit(10).unwrap()
                }
            right = ch.to_digit(10).unwrap()
        }
    }

    right + (left * 10)
}
