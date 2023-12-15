pub type Digits = Vec<Option<(usize, u32)>>;

pub fn read_digits(line: String) -> Digits {
    let chars = line.chars();
    let mut carry = 0_u32;
    let mut pos = None::<usize>;
    let mut digits = Digits::new();
    for (i, ch) in chars.enumerate() {
        match ch {
            x if x.is_ascii_digit() => {
                if pos.is_none() {
                    pos = Some(i);
                }

                carry *= 10;
                carry += x.to_digit(10).unwrap();
            }
            _ => {
                if let Some(v) = pos {
                    for _ in v..i {
                        digits.push(Some((v, carry)));
                    }
                    carry = 0;
                    pos = None::<usize>;
                }
                digits.push(None);
            }
        }
    }

    if let Some(v) = pos {
        for _ in v..line.len() {
            digits.push(Some((v, carry)));
        }
    }

    digits
}

pub fn find_symbols(line: String) -> Vec<usize> {
    line.chars()
        .enumerate()
        .filter(|(_, ch)| !ch.is_ascii_digit() && *ch != '.')
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

pub fn check_digits(line_symbols: Vec<Vec<usize>>, line_digits: Vec<Digits>) -> Vec<u32> {
    let mut nums = std::collections::HashMap::<(usize, usize), u32>::new();
    for (i, symbols) in line_symbols.iter().enumerate() {
        // i is the line index
        for j in symbols {
            let pos_i = i as i32;
            let pos_j = *j as i32;

            find_digits(&mut nums, &line_digits, (pos_i - 1, pos_j - 1));
            find_digits(&mut nums, &line_digits, (pos_i - 1, pos_j));
            find_digits(&mut nums, &line_digits, (pos_i - 1, pos_j + 1));
            find_digits(&mut nums, &line_digits, (pos_i, pos_j - 1));
            find_digits(&mut nums, &line_digits, (pos_i, pos_j + 1));
            find_digits(&mut nums, &line_digits, (pos_i + 1, pos_j - 1));
            find_digits(&mut nums, &line_digits, (pos_i + 1, pos_j));
            find_digits(&mut nums, &line_digits, (pos_i + 1, pos_j + 1));
        }
    }

    nums.values().cloned().collect()
}
