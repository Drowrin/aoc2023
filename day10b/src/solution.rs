#[derive(Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn get(
        self,
        grid: &Vec<Vec<char>>,
        (row, col): (usize, usize),
    ) -> Option<((usize, usize), char)> {
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

fn next(tile: char, from: Direction) -> Option<Direction> {
    let from = from.opposite();
    match tile {
        '|' => match from {
            Direction::N => Some(Direction::S),
            Direction::S => Some(Direction::N),
            _ => None,
        },
        '-' => match from {
            Direction::W => Some(Direction::E),
            Direction::E => Some(Direction::W),
            _ => None,
        },
        'L' => match from {
            Direction::N => Some(Direction::E),
            Direction::E => Some(Direction::N),
            _ => None,
        },
        'J' => match from {
            Direction::N => Some(Direction::W),
            Direction::W => Some(Direction::N),
            _ => None,
        },
        '7' => match from {
            Direction::W => Some(Direction::S),
            Direction::S => Some(Direction::W),
            _ => None,
        },
        'F' => match from {
            Direction::E => Some(Direction::S),
            Direction::S => Some(Direction::E),
            _ => None,
        },
        _ => None,
    }
}

pub fn solution(input: &str) -> impl ToString {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
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
            '.' => false,
            _ => true,
        })
        .collect::<Vec<_>>();

    let mut valid_starters = starters.iter().flat_map(|(d, _, tile)| {
        if next(*tile, *d).is_some() {
            Some(d)
        } else {
            None
        }
    });

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
        .flat_map(|(d, pos, tile)| Some((pos, next(tile, d)?)))
        .next()
        .unwrap();

    let mut loop_positions = vec![(s_row, s_col), starter];
    let mut next_dir = start_dir;

    loop {
        let (pos, tile) = next_dir
            .get(&grid, *loop_positions.last().unwrap())
            .unwrap();

        if pos == loop_positions[0] {
            break;
        } else {
            loop_positions.push(pos);
            next_dir = next(tile, next_dir).unwrap();
        }
    }

    let mut counter = 0;
    for r in 0..grid.len() {
        let mut inside = false;
        let mut span_start: Option<char> = None;

        for c in 0..grid[0].len() {
            if loop_positions.contains(&(r, c)) {
                match grid[r][c] {
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
