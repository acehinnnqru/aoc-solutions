use year_2023::day5part1::{check_seed_location, read_data};
use year_2023::day5part2::read_seeds;
use year_2023::execution::readlines;

// FIXME: this doesn't work properly cause the data is so big.
fn run(p: &str) -> u64 {
    let lines = readlines(p);
    let seeds = read_seeds(&lines[0]);
    let kvmaps = read_data(&lines[2..]);

    let mut min = 0_u64;
    for (start, length) in seeds.iter() {
        for i in 0..*length {
            min = min.min(check_seed_location(*start + i, &kvmaps))
        }
    }

    min
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
        assert!(run("./src/day5part1/sample.txt") == 46);
    }
}
