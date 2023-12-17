use year_2023::day5part1::{check_seed_location, read_data, read_seeds};
use year_2023::execution::readlines;

fn run(p: &str) -> u64 {
    let lines = readlines(p);
    let seeds = read_seeds(&lines[0]);
    let kvmaps = read_data(&lines[2..]);

    seeds
        .iter()
        .map(|seed| check_seed_location(*seed, &kvmaps))
        .min()
        .unwrap()
}

fn main() {
    let min = run("./src/day5part1/data.txt");
    println!("min: {min}");
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn day5part1() {
        assert!(run("./src/day5part1/sample.txt") == 35);
    }
}
