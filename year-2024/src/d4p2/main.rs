use std::{
    char,
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

type Coord = (usize, usize);

type XWord = (Coord, Coord, Coord, Coord, Coord);

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let board = lines
        .map_while(Result::ok)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let m = board.len();
    let n = board[0].len();

    let mut exists = HashMap::<XWord, bool>::new();

    for i in 0..m {
        for j in 0..n {
            if let Some(w) = x(&board, (i, j)) {
                println!("h: {:?}", w);
                exists.insert(w, true);
            }
        }
    }

    println!("{:?}", exists.len());
}

fn checkword(board: &[Vec<char>], word: XWord) -> bool {
    if !view(board, word.2, 'A') {
        return false;
    }

    let mut count = 0;

    if view(board, word.0, 'M') && view(board, word.4, 'S') {
        count += 1
    }

    if view(board, word.0, 'S') && view(board, word.4, 'M') {
        count += 1
    }

    if view(board, word.1, 'S') && view(board, word.3, 'M') {
        count += 1
    }

    if view(board, word.1, 'M') && view(board, word.3, 'S') {
        count += 1
    }

    count >= 2
}

fn view(board: &[Vec<char>], (i, j): Coord, ch: char) -> bool {
    board[i][j] == ch
}

fn x(board: &[Vec<char>], (i, j): Coord) -> Option<XWord> {
    if i == 0 || j == 0 || i + 1 >= board.len() || j + 1 >= board[0].len() {
        return None;
    }

    let w = (
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i, j),
        (i + 1, j - 1),
        (i + 1, j + 1),
    );

    if checkword(board, w) {
        Some(w)
    } else {
        None
    }
}
