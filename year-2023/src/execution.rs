use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn line_run<T, F>(path: &str, f: F) -> Vec<T>
where
    F: Fn(String) -> T,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            f(line)
        })
        .collect()
}
