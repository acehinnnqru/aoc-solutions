use std::{
    collections::BinaryHeap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();

    let lines = BufReader::new(file).lines();

    let mut leftheap = BinaryHeap::<i32>::new();
    let mut rightheap = BinaryHeap::<i32>::new();

    let mut count = 0;
    // build heaps
    lines.map_while(Result::ok).for_each(|s| {
        let (l, r) = s.split_once(" ").unwrap();
        leftheap.push(l.trim().parse::<i32>().unwrap());
        rightheap.push(r.trim().parse::<i32>().unwrap());

        count += 1;
    });

    let mut ret = 0;

    while count > 0 {
        ret += (leftheap.pop().unwrap() - rightheap.pop().unwrap()).abs();
        count -= 1;
    }

    println!("{}", ret);
}
