#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn get(
        self,
        grid: &Vec<Vec<Tile>>,
        (row, col): (usize, usize),
    ) -> Option<((usize, usize), Tile)> {
        let (n_row, n_col) = match self {
            Direction::N => {
                if row == 0 {
                    return None;
                } else {
                    (row - 1, col)
                }
            }
            Direction::E => (row, col + 1),
            Direction::S => (row + 1, col),
            Direction::W => {
                if col == 0 {
                    return None;
                } else {
                    (row, col - 1)
                }
            }
        };

        Some(((n_row, n_col), *grid.get(n_row)?.get(n_col)?))
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Pipe(Direction::N, Direction::S),
            '-' => Self::Pipe(Direction::W, Direction::E),
            'L' => Self::Pipe(Direction::N, Direction::E),
            'J' => Self::Pipe(Direction::N, Direction::W),
            '7' => Self::Pipe(Direction::S, Direction::W),
            'F' => Self::Pipe(Direction::S, Direction::E),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("unrecognized character {c}"),
        }
    }

    fn next(&self, from: Direction) -> Option<Direction> {
        match self {
            Tile::Pipe(d1, d2) => match from.opposite() {
                f if f == *d1 => Some(*d2),
                f if f == *d2 => Some(*d1),
                _ => None,
            },
            Tile::Ground => None,
            Tile::Start => None,
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    let grid: Vec<Vec<Tile>> = input
        .split("\n")
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();

    let start_ind = input.find('S').unwrap();
    let columns = grid[0].len() + 1;
    let s_row = start_ind / columns;
    let s_col = start_ind % columns;

    let (starter, start_dir) = vec![Direction::N, Direction::E, Direction::S, Direction::W]
        .iter()
        .flat_map(|d| {
            let (pos, tile) = d.get(&grid, (s_row, s_col))?;
            Some((d, pos, tile))
        })
        .filter(|(_, _, tile)| match tile {
            Tile::Pipe(_, _) => true,
            _ => false,
        })
        .flat_map(|(d, pos, tile)| Some((pos, tile.next(*d)?)))
        .next()
        .unwrap();

    let mut loop_positions = vec![starter];
    let mut next_dir = start_dir;

    loop {
        let (pos, tile) = next_dir
            .get(&grid, *loop_positions.last().unwrap())
            .unwrap();

        if let Tile::Start = tile {
            break;
        } else {
            loop_positions.push(pos);
            next_dir = tile.next(next_dir).unwrap();
        }
    }

    (loop_positions.len() + 1) / 2
}
