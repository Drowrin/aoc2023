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

    fn collect_area<'s>(&'s self, area: SchematicArea) -> Vec<char> {
        self.grid
            .iter()
            .skip(area.row_min)
            .take(1 + area.row_max - area.row_min)
            .flat_map(move |line| {
                line.iter()
                    .skip(area.col_min)
                    .take(1 + area.col_max - area.col_min)
            })
            .map(|c| *c)
            .collect_vec()
    }
}

pub fn solution(input: &str) -> impl ToString {
    let schematic = Schematic::parse(input);
    schematic
        .find_targets(char::is_ascii_digit)
        .iter()
        .map(|(target, area)| (target, area.expand(schematic.height, schematic.width)))
        .map(|(target, area)| (target, schematic.collect_area(area)))
        .filter(|(_, area_data)| area_data.iter().any(|c| *c != '.' && !c.is_ascii_digit()))
        .map(|(target, _)| target.parse::<u32>().unwrap())
        .sum::<u32>()
}
