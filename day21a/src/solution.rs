use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

fn expand(r: usize, c: usize, max: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    if r > 0 {
        out.push((r - 1, c));
    }
    if r < max {
        out.push((r + 1, c));
    }
    if c > 0 {
        out.push((r, c - 1));
    }
    if c < max {
        out.push((r, c + 1));
    }
    out
}

pub fn solution(input: &str) -> impl ToString {
    let (steps, input) = input.split_once("\n").unwrap();
    let steps = steps.parse::<usize>().unwrap();

    let mut start_pos = (0, 0);
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '.' => Tile::Plot,
                    '#' => Tile::Rock,
                    _ => {
                        start_pos = (r, c);
                        Tile::Plot
                    }
                })
                .collect()
        })
        .collect();
    let max = grid.len() - 1;

    let mut positions = HashSet::from([start_pos]);

    for _ in 0..steps {
        positions = positions
            .into_iter()
            .flat_map(|(r, c)| {
                expand(r, c, max)
                    .into_iter()
                    .filter(|(nr, nc)| grid[*nr][*nc] == Tile::Plot)
            })
            .collect();
    }

    positions.len()
}
