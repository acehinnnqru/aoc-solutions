use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut afters = HashMap::<i32, HashSet<i32>>::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.trim().is_empty() {
            break;
        }

        let (former, latter) = line.split_once("|").unwrap();

        let m = afters.entry(latter.parse().unwrap()).or_default();
        m.insert(former.parse().unwrap());
    }

    println!("{:?}", afters);

    let ret = lines
        .map_while(Result::ok)
        .filter_map(|s| -> Option<i32> {
            let mut exists = HashSet::<i32>::new();
            let mut befores = HashSet::<i32>::new();
            let nums = s
                .split(",")
                .map(|v| {
                    let i = v.parse::<i32>().unwrap();

                    exists.insert(i);

                    i
                })
                .collect::<Vec<i32>>();

            let mut flag = true;
            for n in &nums {
                if let Some(after) = afters.get(n) {
                    for _v in after {
                        if exists.contains(_v) && !befores.contains(_v) {
                            println!("failed: {}", _v);
                            flag = false;
                            break;
                        }
                    }
                }
                befores.insert(*n);
            }

            if flag {
                let ret = nums[nums.len() / 2];
                println!("mid: {}", ret);
                Some(ret)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("{}", ret);
}
