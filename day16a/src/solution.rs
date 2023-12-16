use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(&self, grid: &Vec<Vec<char>>, (row, col): (usize, usize)) -> Option<Beam> {
        match self {
            Self::North => {
                if row == 0 {
                    None
                } else {
                    Some(Beam {
                        loc: (row - 1, col),
                        direction: *self,
                    })
                }
            }
            Self::East => {
                if col >= grid[0].len() - 1 {
                    None
                } else {
                    Some(Beam {
                        loc: (row, col + 1),
                        direction: *self,
                    })
                }
            }
            Self::South => {
                if row >= grid.len() - 1 {
                    None
                } else {
                    Some(Beam {
                        loc: (row + 1, col),
                        direction: *self,
                    })
                }
            }
            Self::West => {
                if col == 0 {
                    None
                } else {
                    Some(Beam {
                        loc: (row, col - 1),
                        direction: *self,
                    })
                }
            }
        }
    }

    fn forward_reflect(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::North,
            Self::South => Self::West,
            Self::West => Self::South,
        }
    }

    fn backward_reflect(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::South,
            Self::South => Self::East,
            Self::West => Self::North,
        }
    }

    fn vertical_split(&self) -> Vec<Self> {
        match self {
            Self::East | Self::West => vec![Self::North, Self::South],
            _ => vec![*self],
        }
    }

    fn horizontal_split(&self) -> Vec<Self> {
        match self {
            Self::North | Self::South => vec![Self::West, Self::East],
            _ => vec![*self],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    direction: Direction,
    loc: (usize, usize),
}

impl Beam {
    fn next(&self, grid: &Vec<Vec<char>>) -> Vec<Beam> {
        let current = grid[self.loc.0][self.loc.1];
        match current {
            '.' => self.direction.next(grid, self.loc).into_iter().collect(),
            '/' => self
                .direction
                .forward_reflect()
                .next(grid, self.loc)
                .into_iter()
                .collect(),
            '\\' => self
                .direction
                .backward_reflect()
                .next(grid, self.loc)
                .into_iter()
                .collect(),
            '|' => self
                .direction
                .vertical_split()
                .iter()
                .map(|d| d.next(grid, self.loc))
                .flatten()
                .collect(),
            '-' => self
                .direction
                .horizontal_split()
                .iter()
                .map(|d| d.next(grid, self.loc))
                .flatten()
                .collect(),
            _ => unreachable!(),
        }
    }
}

fn get_energized(grid: &Vec<Vec<char>>, starter: Beam) -> usize {
    let mut active_beams = vec![starter];
    let mut cache = HashSet::from([starter]);

    while active_beams.len() > 0 {
        active_beams = active_beams
            .iter()
            .flat_map(|beam| beam.next(&grid))
            .collect();

        active_beams = active_beams
            .into_iter()
            .filter(|beam| cache.insert(*beam))
            .collect();
    }

    let energized_locations = cache
        .iter()
        .map(|b| b.loc)
        .collect::<HashSet<(usize, usize)>>();

    energized_locations.len()
}

pub fn solution(input: &str) -> impl ToString {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let starter = Beam {
        direction: Direction::East,
        loc: (0, 0),
    };

    get_energized(&grid, starter)
}
