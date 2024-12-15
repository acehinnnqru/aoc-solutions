use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    // build map of right list and collect left list
    let mut leftvec = Vec::<i32>::new();
    let mut rightcounter = HashMap::<i32, i32>::new();

    // build heaps
    lines.map_while(Result::ok).for_each(|s| {
        let (l, r) = s.split_once(" ").unwrap();
        leftvec.push(l.trim().parse::<i32>().unwrap());
        *rightcounter
            .entry(r.trim().parse::<i32>().unwrap())
            .or_insert(0) += 1;
    });

    let ret = leftvec
        .iter()
        .map(|v| v * *rightcounter.entry(*v).or_default())
        .sum::<i32>();

    println!("{}", ret);
}
