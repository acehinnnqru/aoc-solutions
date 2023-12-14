use year_2023::day1part1::get_calibration_value;

use year_2023::execution::line_run;

fn main() {
    let sum = line_run("./src/day1part1/data.txt", |line| {
        get_calibration_value(line)
    })
    .iter()
    .sum::<u32>();

    println!("sum: {sum}")
}
