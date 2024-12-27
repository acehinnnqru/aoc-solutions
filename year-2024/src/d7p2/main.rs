use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let ret = lines
        .map_while(Result::ok)
        .map(extract)
        .map(|(target, nums)| check(target, nums))
        .filter(|v| v.is_some())
        .sum::<Option<i64>>()
        .unwrap();

    println!("{ret}");
}

fn extract(s: String) -> (i64, Vec<i64>) {
    let (first, second) = s.split_once(":").unwrap();

    let target = first.parse::<i64>().unwrap();
    let nums = second
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (target, nums)
}

fn check(target: i64, nums: Vec<i64>) -> Option<i64> {
    if backtrace(0, Op::Plus, &nums, &target) {
        return Some(target);
    }

    if backtrace(1, Op::Multi, &nums, &target) {
        return Some(target);
    }

    if backtrace(nums[0], Op::Concat, &nums[1..], &target) {
        return Some(target);
    }

    None
}

enum Op {
    Plus,
    Multi,
    Concat,
}

fn backtrace(pre: i64, op: Op, left: &[i64], target: &i64) -> bool {
    if left.is_empty() {
        return pre == *target;
    }

    match op {
        Op::Plus => {
            if backtrace(pre + left[0], Op::Plus, &left[1..], target) {
                return true;
            }
            if backtrace(pre + left[0], Op::Multi, &left[1..], target) {
                return true;
            }
            if backtrace(pre + left[0], Op::Concat, &left[1..], target) {
                return true;
            }
        }
        Op::Multi => {
            if backtrace(pre * left[0], Op::Plus, &left[1..], target) {
                return true;
            }
            if backtrace(pre * left[0], Op::Multi, &left[1..], target) {
                return true;
            }
            if backtrace(pre * left[0], Op::Concat, &left[1..], target) {
                return true;
            }
        }
        Op::Concat => {
            if backtrace(
                format!("{}{}", pre, left[0]).parse::<i64>().unwrap(),
                Op::Plus,
                &left[1..],
                target,
            ) {
                return true;
            }
            if backtrace(
                format!("{}{}", pre, left[0]).parse::<i64>().unwrap(),
                Op::Multi,
                &left[1..],
                target,
            ) {
                return true;
            }
            if backtrace(
                format!("{}{}", pre, left[0]).parse::<i64>().unwrap(),
                Op::Concat,
                &left[1..],
                target,
            ) {
                return true;
            }
        }
    };

    false
}
