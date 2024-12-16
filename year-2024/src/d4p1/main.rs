use std::{
    char,
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

type Coord = (usize, usize);

type Word = (Coord, Coord, Coord, Coord);

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

    let mut exists = HashMap::<Word, bool>::new();

    for i in 0..m {
        for j in 0..n {
            // horizontal
            if let Some(w) = h(&board, (i, j)) {
                println!("h: {:?}", w);
                exists.insert(w, true);
            }

            // vertical
            if let Some(w) = v(&board, (i, j)) {
                println!("v: {:?}", w);
                exists.insert(w, true);
            }

            // diagonal
            if let Some(w) = d_lr(&board, (i, j)) {
                println!("d-lr: {:?}", w);
                exists.insert(w, true);
            }
            if let Some(w) = d_rl(&board, (i, j)) {
                println!("d-rl: {:?}", w);
                exists.insert(w, true);
            }
        }
    }

    println!("{:?}", exists.len());
}

fn checkword(board: &[Vec<char>], word: Word) -> bool {
    (view(board, word.0, 'X')
        && view(board, word.1, 'M')
        && view(board, word.2, 'A')
        && view(board, word.3, 'S'))
        || (view(board, word.0, 'S')
            && view(board, word.1, 'A')
            && view(board, word.2, 'M')
            && view(board, word.3, 'X'))
}

fn view(board: &[Vec<char>], (i, j): Coord, ch: char) -> bool {
    board[i][j] == ch
}

// horizontal
fn h(board: &[Vec<char>], (i, j): Coord) -> Option<Word> {
    if j + 3 >= board.len() {
        return None;
    }

    let word = ((i, j), (i, j + 1), (i, j + 2), (i, j + 3));

    if checkword(board, word) {
        Some(word)
    } else {
        None
    }
}

// vertical
fn v(board: &[Vec<char>], (i, j): Coord) -> Option<Word> {
    if i + 3 >= board[0].len() {
        return None;
    }

    let word = ((i, j), (i + 1, j), (i + 2, j), (i + 3, j));

    if checkword(board, word) {
        Some(word)
    } else {
        None
    }
}

// diagonal from left to right
fn d_lr(board: &[Vec<char>], (i, j): Coord) -> Option<Word> {
    if i + 3 >= board[0].len() || j + 3 >= board.len() {
        return None;
    }

    let word = ((i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3));

    if checkword(board, word) {
        Some(word)
    } else {
        None
    }
}

// diagonal from right to left
fn d_rl(board: &[Vec<char>], (i, j): Coord) -> Option<Word> {
    if i + 3 >= board.len() || j < 3 {
        return None;
    }

    let word = ((i + 3, j - 3), (i + 2, j - 2), (i + 1, j - 1), (i, j));

    if checkword(board, word) {
        Some(word)
    } else {
        None
    }
}
