pub fn read_seeds(line: &str) -> Vec<(u64, u64)> {
    let mut splits = line.split(':');
    splits
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect()
}
