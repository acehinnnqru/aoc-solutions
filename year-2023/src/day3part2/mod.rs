use super::day3part1::Digits;

pub fn find_gears(line: String) -> Vec<usize> {
    line.chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '*')
        .map(|(i, _)| i)
        .collect()
}

pub fn find_digits(
    nums: &mut std::collections::HashMap<(usize, usize), u32>,
    line_digits: &[Digits],
    pos: (i32, i32),
) {
    if pos.0 < 0
        || pos.0 as usize >= line_digits.len()
        || pos.1 < 0
        || pos.1 as usize >= line_digits[0].len()
    {
        return;
    }

    if let Some((j, v)) = line_digits[pos.0 as usize][pos.1 as usize] {
        nums.insert((pos.0 as usize, j), v);
    }
}

pub fn find_gears_ratios(line_symbols: Vec<Vec<usize>>, line_digits: Vec<Digits>) -> Vec<u32> {
    let mut ratios = Vec::<u32>::new();
    for (i, symbols) in line_symbols.iter().enumerate() {
        // i is the line index
        for j in symbols {
            let pos_i = i as i32;
            let pos_j = *j as i32;
            let mut tmp_nums = std::collections::HashMap::<(usize, usize), u32>::new();

            find_digits(&mut tmp_nums, &line_digits, (pos_i - 1, pos_j - 1));
            find_digits(&mut tmp_nums, &line_digits, (pos_i - 1, pos_j));
            find_digits(&mut tmp_nums, &line_digits, (pos_i - 1, pos_j + 1));
            find_digits(&mut tmp_nums, &line_digits, (pos_i, pos_j - 1));
            find_digits(&mut tmp_nums, &line_digits, (pos_i, pos_j + 1));
            find_digits(&mut tmp_nums, &line_digits, (pos_i + 1, pos_j - 1));
            find_digits(&mut tmp_nums, &line_digits, (pos_i + 1, pos_j));
            find_digits(&mut tmp_nums, &line_digits, (pos_i + 1, pos_j + 1));

            if tmp_nums.len() == 2 {
                ratios.push(tmp_nums.values().product::<u32>())
            }
        }
    }

    ratios
}
