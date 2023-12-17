use year_2023::day4part1::parse_winning_numbers;
use year_2023::execution::line_run;

fn run(p: &str) -> u32 {
    line_run(p, parse_winning_numbers)
        .iter()
        .map(|v| {
            if v.is_empty() {
                0
            } else {
                2_u32.pow(v.len() as u32 - 1)
            }
        })
        .sum()
}

fn main() {
    let sum = run("./src/day4part1/data.txt");

    println!("sum: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn day4part1() {
        assert!(run("./src/day4part1/sample.txt") == 13);
    }
}
