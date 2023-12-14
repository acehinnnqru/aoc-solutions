use std::{io::{BufReader, BufRead}, fs::File, path::PathBuf};

pub fn read_lines(p: PathBuf) -> std::io::Lines<BufReader<File>> {
    let file = File::open(p).unwrap();

    let reader = BufReader::new(file);

    reader.lines()
}

pub fn get_calibration_value(line: String) -> u32 {
    let mut left = 0_u32;
    let mut right = 0_u32;
    for ch in line.chars() {
        if ch.is_ascii_digit() {
            left = ch.to_digit(10).unwrap();
            break;
        }
    }

    for ch in line.chars().rev() {
        if ch.is_ascii_digit() {
            right = ch.to_digit(10).unwrap();
            break;
        }
    }

    left * 10 + right
}

