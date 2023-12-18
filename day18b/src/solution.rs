enum Direction {
    North,
    East,
    South,
    West,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "3" => Self::North,
            "0" => Self::East,
            "1" => Self::South,
            "2" => Self::West,
            _ => unreachable!(),
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let hex_index = line.find('#').unwrap();
            let distance = &line[hex_index + 1..hex_index + 6];
            let direction = &line[hex_index + 6..hex_index + 7];

            (
                Direction::from(direction),
                isize::from_str_radix(distance, 16).unwrap(),
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
