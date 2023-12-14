use year_2023::day2part1::{check_cubes, read_game, CubeMap};
use year_2023::execution::line_run;

fn run(path: &str) -> u32 {
    let mut expect_map = CubeMap::new();

    expect_map.insert("blue".to_string(), 14);
    expect_map.insert("green".to_string(), 13);
    expect_map.insert("red".to_string(), 12);

    line_run(path, |line| -> bool {
        let cubes = read_game(line);
        check_cubes(&cubes, &expect_map)
    })
    .iter()
    .enumerate()
    .filter(|(_, b)| **b)
    .map(|(i, _)| i as u32 + 1)
    .sum::<u32>()
}

fn main() {
    let sum = run("./src/day2part1/data.txt");
    println!("sum: {sum}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn day2part1() {
        assert!(crate::run("./src/day2part1/sample.txt") == 8);
    }
}
