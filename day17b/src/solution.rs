use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}

impl Direction {
    fn next(&self, node: &Node, max_row: usize, max_col: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => {
                if node.pos.0 == 0 {
                    None
                } else {
                    Some((node.pos.0 - 1, node.pos.1))
                }
            }
            Direction::East => {
                if node.pos.1 == max_col {
                    None
                } else {
                    Some((node.pos.0, node.pos.1 + 1))
                }
            }
            Direction::South => {
                if node.pos.0 == max_row {
                    None
                } else {
                    Some((node.pos.0 + 1, node.pos.1))
                }
            }
            Direction::West => {
                if node.pos.1 == 0 {
                    None
                } else {
                    Some((node.pos.0, node.pos.1 - 1))
                }
            }
            Direction::None => None,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::None => Direction::None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Node {
    pos: (usize, usize),
    straight: usize,
    dir: Direction,
}

fn display(input: &str, nodes: Vec<Node>) {
    let mut g: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    for r in nodes.iter() {
        g[r.pos.0][r.pos.1] = format!(
            "\x1b[0;32m{}\x1b[0m",
            match r.dir {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
                Direction::None => 'N',
            }
        )
    }
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            print!("{}", g[r][c]);
        }
        println!();
    }
}

pub fn solution(input: &str) -> impl ToString {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;

    let matrix = Matrix::from_rows(grid).unwrap();

    let goal = (max_row, max_col);

    let start = Node {
        pos: (0, 0),
        straight: 0,
        dir: Direction::None,
    };

    let successors = |node: &Node| {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .filter(|d| *d != node.dir.opposite())
        .filter(|d| node.straight >= 4 || *d == node.dir || node.dir == Direction::None)
        .flat_map(|d| d.next(node, max_row, max_col).map(|v| (d, v)))
        .map(|(d, (r, c))| {
            (
                Node {
                    pos: (r, c),
                    straight: if node.dir == d { node.straight + 1 } else { 1 },
                    dir: d,
                },
                *matrix.get((r, c)).unwrap(),
            )
        })
        .filter(|(node, _)| node.straight <= 10)
        .collect::<Vec<_>>()
    };

    let success = |node: &Node| node.pos == goal && node.straight >= 4;

    let results = dijkstra(&start, successors, success).unwrap();

    display(input, results.0);

    results.1
}
