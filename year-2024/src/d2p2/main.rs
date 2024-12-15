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

    // build heaps
    let count = lines
        .map_while(Result::ok)
        .filter(|s| {
            let nums = s
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let (safe, index) = is_safe(nums.iter());
            if safe {
                return true;
            }

            if let Some(index) = index {
                // remove elem in pos index
                let newnums = [&nums[..index], &nums[index + 1..]].concat();
                // println!("1 {:?}", newnums);
                let (safe, _) = is_safe(newnums.iter());
                if safe {
                    return true;
                }

                // remove elem in pos index - 1
                if index <= 1 {
                    let newnums = [&nums[index..]].concat();
                    let (safe, _) = is_safe(newnums.iter());
                    // println!("2 {:?}", newnums);

                    if safe {
                        return true;
                    }
                } else {
                    let newnums = [&nums[..index - 1], &nums[index..]].concat();
                    // println!("3 {:?}", newnums);
                    let (safe, _) = is_safe(newnums.iter());
                    if safe {
                        return true;
                    }
                };

                // remove elem in pos index - 2
                return if index <= 2 {
                    let newnums = [&nums[index - 1..]].concat();
                    let (safe, _) = is_safe(newnums.iter());
                    // println!("2 {:?}", newnums);

                    safe
                } else {
                    let newnums = [&nums[..index - 2], &nums[index..]].concat();
                    // println!("3 {:?}", newnums);
                    let (safe, _) = is_safe(newnums.iter());
                    safe
                };
            } else {
                panic!("")
            };
        })
        .count();

    println!("{}", count);
}

fn is_safe<'a, I>(mut nums: I) -> (bool, Option<usize>)
where
    I: Iterator<Item = &'a i32>,
{
    let first = nums.next().unwrap();
    let second = nums.next().unwrap();

    let flag = second - first > 0;

    if !is_pair_safe(flag, second, first) {
        return (false, Some(1));
    }

    // first and second must be safe
    let mut pre = second;
    for (ind, cur) in nums.enumerate() {
        if !is_pair_safe(flag, cur, pre) {
            return (false, Some(ind + 2));
        }

        pre = cur
    }

    (true, None)
}

fn is_pair_safe(flag: bool, a: &i32, b: &i32) -> bool {
    let diff = a - b;

    diff != 0 && diff.abs() <= 3 && flag == (diff > 0)
}
