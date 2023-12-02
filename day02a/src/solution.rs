use itertools::Itertools;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

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

    fn validate(&self) -> bool {
        self.r <= MAX_RED && self.g <= MAX_GREEN && self.b <= MAX_BLUE
    }
}

struct Game {
    id: i32,
    cubeset: Vec<CubeSet>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game, sets) = line.split(":").collect_tuple().unwrap();
        Game {
            id: game[5..].parse::<i32>().unwrap(),
            cubeset: sets.split(";").map(CubeSet::parse).collect_vec(),
        }
    }

    fn validate(&self) -> bool {
        self.cubeset.iter().all(CubeSet::validate)
    }
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(Game::parse)
        .filter(Game::validate)
        .map(|g| g.id)
        .sum::<i32>()
}
