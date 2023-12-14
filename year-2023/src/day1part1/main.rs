use std::path::PathBuf;

use year_2023::day1part1::get_calibration_value;
use year_2023::day1part1::read_lines;

fn main() {
    let mut sum = 0_u32;
    for line in read_lines(PathBuf::from("./src/day1part1/data.txt")) {
        sum += get_calibration_value(line.unwrap());
    }

    println!("sum: {sum}")
}
