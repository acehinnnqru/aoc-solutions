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
        .map(|s| -> (String, bool) {
            let mut ns = s.split_whitespace();
            let first = ns.next().unwrap().parse::<i32>().unwrap();
            let second = ns.next().unwrap().parse::<i32>().unwrap();

            let diff = second - first;
            if diff == 0 || diff.abs() > 3 {
                return (s, false);
            }

            let flag = diff > 0;
            let mut pre = second;

            for v in ns {
                let cur = v.parse::<i32>().unwrap();
                let diff = cur - pre;
                if diff == 0 || diff.abs() > 3 || (diff > 0) != flag {
                    return (s, false);
                }

                pre = cur
            }

            (s, true)
        })
        .map(|(s, v)| -> bool {
            if !v {
                println!("{}", s);
            }
            v
        })
        .filter(|v| *v)
        .count();

    println!("{}", count);
}
