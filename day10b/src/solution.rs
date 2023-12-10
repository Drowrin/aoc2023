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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: &char) -> Self {
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

    fn next(&self, from: Direction) -> Result<Direction, String> {
        match self {
            Tile::Pipe(d1, d2) => match from.opposite() {
                f if f == *d1 => Ok(*d2),
                f if f == *d2 => Ok(*d1),
                f => Err(format!("direction {f:?} did not match {d1:?} or {d2:?}")),
            },
            Tile::Ground => Err(format!("Tried to get next direction for ground")),
            Tile::Start => Err(format!("Tried to get next direction for start")),
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    let o_grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let grid: Vec<Vec<Tile>> = o_grid
        .iter()
        .map(|line| line.iter().map(Tile::from_char).collect())
        .collect();

    let start_ind = input.find('S').unwrap();
    let columns = grid[0].len() + 1;
    let s_row = start_ind / columns;
    let s_col = start_ind % columns;

    let starters = vec![Direction::N, Direction::E, Direction::S, Direction::W]
        .into_iter()
        .flat_map(|d| {
            let (pos, tile) = d.get(&grid, (s_row, s_col))?;
            Some((d, pos, tile))
        })
        .filter(|(_, _, tile)| match tile {
            Tile::Pipe(_, _) => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    let mut valid_starters =
        starters
            .iter()
            .flat_map(|(d, _, tile)| if tile.next(*d).is_ok() { Some(d) } else { None });
    let start_shape = match (
        valid_starters.next().unwrap(),
        valid_starters.next().unwrap(),
    ) {
        (Direction::N, Direction::E) => 'L',
        (Direction::N, Direction::S) => '|',
        (Direction::N, Direction::W) => 'J',
        (Direction::E, Direction::N) => 'L',
        (Direction::E, Direction::S) => 'F',
        (Direction::E, Direction::W) => '-',
        (Direction::S, Direction::N) => '|',
        (Direction::S, Direction::E) => 'F',
        (Direction::S, Direction::W) => '7',
        (Direction::W, Direction::N) => 'J',
        (Direction::W, Direction::E) => '-',
        (Direction::W, Direction::S) => '7',
        _ => unreachable!(),
    };

    let (starter, start_dir) = starters
        .into_iter()
        .flat_map(
            |(d, pos, tile)| -> Result<((usize, usize), Direction), String> {
                Ok((pos, tile.next(d)?))
            },
        )
        .next()
        .unwrap();

    let mut loop_positions = vec![(s_row, s_col), starter];
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

    let mut counter = 0;
    for r in 0..o_grid.len() {
        let mut inside = false;
        let mut span_start: Option<char> = None;

        for c in 0..o_grid[0].len() {
            if loop_positions.contains(&(r, c)) {
                match o_grid[r][c] {
                    '|' => {
                        inside = !inside;
                    }
                    'L' => {
                        span_start = Some('L');
                    }
                    'F' => {
                        span_start = Some('F');
                    }
                    'S' => match start_shape {
                        'F' => {
                            span_start = Some('F');
                        }
                        'L' => {
                            span_start = Some('L');
                        }
                        _ => {}
                    },
                    '7' => match span_start {
                        Some('L') => {
                            inside = !inside;
                        }
                        Some('F') => {}
                        _ => panic!("reached span end without span start"),
                    },
                    'J' => match span_start {
                        Some('F') => {
                            inside = !inside;
                        }
                        Some('L') => {}
                        _ => panic!("reached span end without span start"),
                    },
                    _ => {}
                }
            } else {
                if inside {
                    counter += 1;
                }
            }
        }
    }

    counter
}
