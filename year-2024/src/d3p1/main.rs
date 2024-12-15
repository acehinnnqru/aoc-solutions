use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    // build heaps
    let ret = lines
        .map_while(Result::ok)
        .map(|s| -> i32 {
            let r = Regex::new(r"mul\((?P<left>[0-9]+)\,(?P<right>[0-9]+)\)").unwrap();

            r.captures_iter(&s)
                .map(|caps| -> i32 {
                    let left = &caps["left"].parse::<i32>().unwrap();
                    let right = &caps["right"].parse::<i32>().unwrap();

                    left * right
                })
                .sum()
        })
        .sum::<i32>();

    println!("{}", ret);
}
