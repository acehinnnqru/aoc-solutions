use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug)]
enum Op {
    Donot,
    Do,
    Mul(i32, i32),
}

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    // build heaps
    let (_, ret) =
        lines
            .map_while(Result::ok)
            .fold((true, 0), |(enabled, sum), s| -> (bool, i32) {
                let r =
                    Regex::new(r"(mul\((?P<left>[0-9]+)\,(?P<right>[0-9]+)\))|(do\(\))|don't\(\)")
                        .unwrap();

                r.captures_iter(&s)
                    .map(|caps| -> Op {
                        match &caps[0] {
                            "do()" => Op::Do,
                            "don't()" => Op::Donot,
                            _ => {
                                let left = &caps["left"].parse::<i32>().unwrap();
                                let right = &caps["right"].parse::<i32>().unwrap();
                                Op::Mul(*left, *right)
                            }
                        }
                    })
                    .fold((enabled, sum), |(enabled, sum), op| -> (bool, i32) {
                        println!("{:?} {}", op, sum);
                        match op {
                            Op::Do => (true, sum),
                            Op::Donot => (false, sum),
                            Op::Mul(x, y) => {
                                if !enabled {
                                    (enabled, sum)
                                } else {
                                    (enabled, sum + x * y)
                                }
                            }
                        }
                    })
            });

    println!("{}", ret);
}
