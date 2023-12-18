use itertools::Itertools;

enum Direction {
    North,
    East,
    South,
    West,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::North,
            "R" => Self::East,
            "D" => Self::South,
            "L" => Self::West,
            _ => unreachable!(),
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let (direction, distance, _) = line.split(" ").collect_tuple().unwrap();
            (
                Direction::from(direction),
                distance.parse::<isize>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(a, r, c), (d, n)| match d {
            Direction::North => (a + n - 2 * c * n, r - n, c),
            Direction::East => (a + n, r, c + n),
            Direction::South => (a + n + 2 * c * n, r + n, c),
            Direction::West => (a + n, r, c - n),
        })
        .0
        / 2
        + 1
}
