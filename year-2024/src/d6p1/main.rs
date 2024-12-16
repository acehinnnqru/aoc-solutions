use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

// a < b
type Coord = (usize, usize);

struct Board {
    m: usize,
    n: usize,
    board: Vec<Vec<char>>,
}

impl Board {
    fn new(board: Vec<Vec<char>>) -> Self {
        Board {
            m: board.len(),
            n: board[0].len(),
            board,
        }
    }

    fn origin(&self) -> Coord {
        let mut start = Coord::default();
        for i in 0..self.m {
            for j in 0..self.n {
                if self.board[i][j] == '^' {
                    start = (i, j);
                    break;
                }
            }
        }

        start
    }

    fn is_obstacle(&self, c: Coord) -> bool {
        self.board[c.0][c.1] == '#'
    }

    fn next(&self, d: &Direction, cur: Coord) -> Option<Coord> {
        let (i, j) = cur;
        match d {
            Direction::Up => (i >= 1).then(|| (i - 1, j)),
            Direction::Right => (j < self.n - 1).then(|| (i, j + 1)),
            Direction::Down => (i < self.m - 1).then(|| (i + 1, j)),
            Direction::Left => (j >= 1).then(|| (i, j - 1)),
        }
    }
}

fn main() {
    // read data file
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let board = lines
        .map_while(Result::ok)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let board = Board::new(board);
    // the origin coord
    let origin = board.origin();

    let mut viewed = HashSet::<Coord>::new();
    viewed.insert(origin);

    view(&board, origin, &mut viewed);

    println!("{}", viewed.len());
}

fn view(board: &Board, origin: Coord, viewed: &mut HashSet<Coord>) {
    let mut d = Direction::Up;
    let mut cur = origin;

    // next got value when inside board
    while let Some(c) = board.next(&d, cur) {
        if board.is_obstacle(c) {
            d = d.turn();
        } else {
            viewed.insert(c);
            cur = c;
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
