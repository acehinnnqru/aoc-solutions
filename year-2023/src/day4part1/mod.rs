use std::collections::HashSet;

pub fn parse_winning_numbers(line: String) -> Vec<u32> {
    let line = line.split(':').nth(1).unwrap();
    let mut splits = line.split('|');
    let targets: HashSet<&str> = splits.next().unwrap().split_whitespace().collect();
    let gots: Vec<&str> = splits.next().unwrap().split(' ').collect();

    gots.into_iter()
        .filter(|v| targets.contains(v))
        .map(|v| v.parse().unwrap())
        .collect()
}
