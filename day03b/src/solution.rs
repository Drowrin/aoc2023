use std::ops::Add;

use itertools::Itertools;

struct SchematicArea {
    row_min: usize,
    row_max: usize,
    col_min: usize,
    col_max: usize,
}

impl SchematicArea {
    fn expand(&self, schematic_height: usize, schematic_width: usize) -> Self {
        Self {
            row_min: self.row_min.saturating_sub(1),
            row_max: self.row_max.add(1).min(schematic_height - 1),
            col_min: self.col_min.saturating_sub(1),
            col_max: self.col_max.add(1).min(schematic_width - 1),
        }
    }

    fn overlap(&self, other: &SchematicArea) -> bool {
        self.row_min <= other.row_max
            && self.row_max >= other.row_min
            && self.col_min <= other.col_max
            && self.col_max >= other.col_min
    }
}

struct Schematic {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Schematic {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input
            .split("\n")
            .map(|line| line.chars().collect())
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }

    fn find_targets<P>(self: &Schematic, predicate: P) -> Vec<(String, SchematicArea)>
    where
        P: Fn(&char) -> bool,
    {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                let mut line_iter = line.iter().peekable();
                let mut col_start = 0;
                let mut out = Vec::new();

                while line_iter.peek().is_some() {
                    col_start += line_iter.peeking_take_while(|c| !predicate(*c)).count();

                    let target: String = line_iter.peeking_take_while(|c| predicate(*c)).collect();

                    let col_end = col_start + target.len() - 1;
                    let target_length = target.len();

                    if target.len() > 0 {
                        out.push((
                            target,
                            SchematicArea {
                                row_min: row,
                                row_max: row,
                                col_min: col_start,
                                col_max: col_end,
                            },
                        ));
                    }

                    col_start += target_length;
                }

                out
            })
            .collect_vec()
    }
}

pub fn solution(input: &str) -> impl ToString {
    let schematic = Schematic::parse(input);
    let numbers = schematic.find_targets(|c| c.is_ascii_digit());
    schematic
        .find_targets(|c| *c == '*')
        .iter()
        .map(|(_, area)| area.expand(schematic.height, schematic.width))
        .map(|area| {
            match numbers
                .iter()
                .filter(|(_, number_area)| area.overlap(number_area))
                .map(|(number, _)| number.parse::<u32>().unwrap())
                .collect_tuple()
            {
                Some((a, b)) => a * b,
                None => 0,
            }
        })
        .sum::<u32>()
}
