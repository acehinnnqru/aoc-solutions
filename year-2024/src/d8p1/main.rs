use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

type Board = Vec<Vec<char>>;

type Coord = (usize, usize);

type Antennas = HashMap<char, Vec<Coord>>;

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut board = Board::new();
    lines.map_while(Result::ok).for_each({
        |line: String| {
            board.push(line.chars().collect::<Vec<char>>());
        }
    });

    let mut antennas = Antennas::new();
    board.iter().enumerate().for_each(|(i, chars)| {
        chars.iter().enumerate().for_each(|(j, ch)| {
            if *ch != '.' {
                let v = antennas.entry(*ch).or_default();
                v.push((i, j));
            }
        });
    });

    let f = antinode_of(board.len(), board[0].len());
    antennas.iter().for_each(|(_, coords)| {
        coords.iter().permutations(2).for_each(|pairs| {
            let left = pairs[0];
            let right = pairs[1];

            if let Some(pos) = f(left, right) {
                if board[pos.0][pos.1] != '#' {
                    board[pos.0][pos.1] = '#'
                }
            }
        });
    });

    let ret = board
        .iter()
        .map(|v| v.iter().filter(|ch| **ch == '#').count())
        .sum::<usize>();

    println!("{ret}");
}

fn antinode_of(mmax: usize, nmax: usize) -> impl Fn(&Coord, &Coord) -> Option<Coord> {
    println!("{mmax} {nmax}");
    move |left, right| -> Option<Coord> {
        let i = left.0 as i32 - 2 * (left.0 as i32 - right.0 as i32);
        let j = left.1 as i32 - 2 * (left.1 as i32 - right.1 as i32);

        if i < 0 || j < 0 {
            return None;
        }

        let i = i as usize;
        let j = j as usize;

        if i >= mmax || j >= nmax {
            return None;
        }

        Some((i, j))
    }
}
