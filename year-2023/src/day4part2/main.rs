use year_2023::day4part1::parse_winning_numbers;
use year_2023::day4part2::cal_copies;
use year_2023::execution::line_run;

fn run(p: String) -> u32 {
    let winnings = line_run(&p, parse_winning_numbers);
    let copies = cal_copies(&winnings);

    copies.iter().sum::<u32>()
}

fn main() {
    let sum = run("./src/day4part1/data.txt".to_string());

    println!("sum: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn day4part2() {
        assert!(run("./src/day4part1/sample.txt".to_string()) == 30);
    }
}
