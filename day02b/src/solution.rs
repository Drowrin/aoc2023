use itertools::Itertools;

#[derive(Default)]
struct CubeSet {
    r: i32,
    g: i32,
    b: i32,
}

impl CubeSet {
    fn parse(data: &str) -> Self {
        let mut out = CubeSet::default();

        data.split(",")
            .map(|cube| cube.split(" ").skip(1).collect_tuple().unwrap())
            .for_each(|(number, color)| {
                let cube_count = number.parse::<i32>().unwrap();

                match color {
                    "red" => out.r += cube_count,
                    "green" => out.g += cube_count,
                    "blue" => out.b += cube_count,
                    _ => panic!("Unrecognized color: {color}"),
                }
            });

        out
    }

    fn power(self) -> i32 {
        self.r * self.g * self.b
    }
}

struct Game(Vec<CubeSet>);

impl Game {
    fn parse(line: &str) -> Self {
        Game(
            line.split(":")
                .skip(1)
                .next()
                .unwrap()
                .split(";")
                .map(CubeSet::parse)
                .collect_vec(),
        )
    }

    fn minimum_set(self) -> CubeSet {
        CubeSet {
            r: self.0.iter().map(|s| s.r).max().unwrap(),
            g: self.0.iter().map(|s| s.g).max().unwrap(),
            b: self.0.iter().map(|s| s.b).max().unwrap(),
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(Game::parse)
        .map(Game::minimum_set)
        .map(CubeSet::power)
        .sum::<i32>()
}
