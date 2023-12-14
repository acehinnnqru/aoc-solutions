use std::path::PathBuf;

fn match_val(line: &str, i: usize, target: &str, val: u32) -> Option<u32> {
    if line[i..].starts_with(target) {
        Some(val)
    } else {
        None
    }
}

fn get_calibration_value(line: String) -> u32 {
    let mut left = 0_u32;
    let chars = line.chars();
    for (i, ch) in chars.enumerate() {
        if let Some(x) = match ch {
            x if ch.is_ascii_digit() => Some(x.to_digit(10).unwrap()),
            'o' => match_val(&line, i, "one", 1),
            't' => match_val(&line, i, "two", 2).or(match_val(&line, i, "three", 3)),
            'f' => match_val(&line, i, "four", 4).or(match_val(&line, i, "five", 5)),
            's' => match_val(&line, i, "six", 6).or(match_val(&line, i, "seven", 7)),
            'e' => match_val(&line, i, "eight", 8),
            'n' => match_val(&line, i, "nine", 9),
            _ => None,
        } {
            left = x;
            break;
        }
    }

    let mut right = 0_u32;
    let bytes = line.bytes().rev();
    let line = String::from_utf8(bytes.collect()).unwrap();
    let chars = line.chars();
    for (i, ch) in chars.enumerate() {
        if let Some(x) = match ch {
            x if ch.is_ascii_digit() => Some(x.to_digit(10).unwrap()),
            'e' => match_val(&line, i, "eno", 1)
                .or(match_val(&line, i, "enin", 9))
                .or(match_val(&line, i, "eerht", 3))
                .or(match_val(&line, i, "evif", 5)),
            'o' => match_val(&line, i, "owt", 2),
            'r' => match_val(&line, i, "ruof", 4),
            'x' => match_val(&line, i, "xis", 6),
            't' => match_val(&line, i, "thgie", 8),
            'n' => match_val(&line, i, "neves", 7),
            _ => None,
        } {
            right = x;
            break;
        }
    }

    left * 10 + right
}

fn main() {
    use year_2023::day1part1::read_lines;

    let mut sum = 0_u32;
    for line in read_lines(PathBuf::from("./src/day1part1/data.txt")) {
        let line = line.unwrap();
        sum += get_calibration_value(line);
    }

    println!("sum: {sum}")
}
