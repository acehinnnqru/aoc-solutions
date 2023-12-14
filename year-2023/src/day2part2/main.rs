use year_2023::day2part1::read_game;
use year_2023::execution::line_run;

use year_2023::day2part2::{cal_least_map, get_power};

fn run(path: &str) -> u32 {
    line_run(path, |line| -> u32 {
        let cubes = read_game(line);

        get_power(&cal_least_map(cubes))
    })
    .iter()
    .sum::<u32>()
}

fn main() {
    let sum = run("./src/day2part1/data.txt");
    println!("sum: {sum}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn day2part2() {
        assert!(crate::run("./src/day2part1/sample.txt") == 2286);
    }
}
