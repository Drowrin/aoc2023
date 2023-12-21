use std::collections::HashSet;

use polyfit_rs::polyfit_rs::polyfit;

#[derive(PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

fn expand(r: isize, c: isize) -> Vec<(isize, isize)> {
    vec![(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
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
                        start_pos = (r as isize, c as isize);
                        Tile::Plot
                    }
                })
                .collect()
        })
        .collect();

    let max = grid.len() as isize;
    let clamp = |v: isize| -> usize {
        if v >= 0 {
            (v % max) as usize
        } else {
            (((v % max) + max) % max) as usize
        }
    };
    let is_plot = |(r, c): &(isize, isize)| -> bool {
        let nr = clamp(*r);
        let nc = clamp(*c);
        grid[nr as usize][nc as usize] == Tile::Plot
    };

    let mut positions = HashSet::from([start_pos]);

    let mult = max as usize;
    let offset = steps % mult;
    let xs = [
        offset as f64,
        (offset + mult * 2) as f64,
        (offset + mult * 4) as f64,
    ];
    let mut ys = [0.0; 3];
    for i in 0..steps {
        positions = positions
            .into_iter()
            .flat_map(|(r, c)| expand(r, c).into_iter().filter(is_plot))
            .collect();

        if i == xs[0] as usize {
            ys[0] = positions.len() as f64;
        }
        if i == xs[1] as usize {
            ys[1] = positions.len() as f64;
        }
        if i == xs[2] as usize {
            ys[2] = positions.len() as f64;

            println!("{:?} -> {:?}", xs, ys);
            let c = polyfit(&xs, &ys, 2).unwrap();
            println!("{:?}", c);

            let x = steps as f64;
            let y = c[2] * x.powf(2.0) + c[1] * x + c[0];
            println!("{y}");
            return y.round() as usize;
        }
    }

    positions.len()
}
