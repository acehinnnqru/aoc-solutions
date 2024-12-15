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
        .filter(|s| -> bool {
            let mut ns = s.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            let first = ns.next().unwrap();
            let second = ns.next().unwrap();

            let diff = second - first;
            if diff == 0 || diff.abs() > 3 {
                return false;
            }

            let flag = diff > 0;
            let mut pre = second;

            for v in ns {
                let cur = v;
                let diff = cur - pre;
                if diff == 0 || diff.abs() > 3 || (diff > 0) != flag {
                    return false;
                }

                pre = cur
            }

            true
        })
        .count();

    println!("{}", count);
}
