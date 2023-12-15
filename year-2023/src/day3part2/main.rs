use year_2023::day3part1::read_digits;
use year_2023::day3part2::{find_gears, find_gears_ratios};
use year_2023::execution::line_run;

fn run(p: &str) -> u32 {
    let digits = line_run(p, read_digits).to_vec();
    let symbols = line_run(p, find_gears).to_vec();

    let ret_digits = find_gears_ratios(symbols, digits);

    ret_digits.iter().sum::<u32>()
}

fn main() {
    let p = "./src/day3part1/data.txt";

    println!("sum: {}", run(p))
}

#[cfg(test)]
mod tests {
    #[test]
    fn day3part1() {
        use crate::run;
        assert!(run("./src/day3part1/sample.txt") == 467835)
    }
}
