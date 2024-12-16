use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

// a < b
type Less = (i32, i32);
type Rule = HashSet<Less>;

fn less(rule: &Rule, a: i32, b: i32) -> bool {
    rule.contains(&(a, b))
}

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file).lines();

    let rule = build_rules(&mut lines);

    let ret = lines
        .map_while(Result::ok)
        .filter_map(|s| -> Option<i32> {
            let nums = s
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>();

            for i in 1..nums.len() {
                if !less(&rule, nums[i - 1], nums[i]) {
                    return None;
                }
            }

            Some(nums[nums.len() / 2])
        })
        .sum::<i32>();

    println!("{}", ret);
}

fn build_rules(lines: &mut Lines<BufReader<File>>) -> Rule {
    let mut rules = Rule::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.trim().is_empty() {
            break;
        }

        let (former, latter) = line.split_once("|").unwrap();

        rules.insert((former.parse().unwrap(), latter.parse().unwrap()));
    }

    rules
}
