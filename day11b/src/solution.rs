use std::ops::Sub;

pub fn solution(input: &str) -> impl ToString {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row_data)| {
            row_data
                .iter()
                .enumerate()
                .flat_map(|(c, data)| if data.eq(&'#') { Some((r, c)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect();

    for (r, data) in grid.iter().enumerate().rev() {
        if data.iter().all(|d| d.eq(&'.')) {
            for g in galaxies.iter_mut() {
                if g.0 > r {
                    g.0 += 999999;
                }
            }
        }
    }

    for c in (0..grid[0].len()).rev() {
        if grid.iter().map(|row| row[c]).all(|d| d.eq(&'.')) {
            for g in galaxies.iter_mut() {
                if g.1 > c {
                    g.1 += 999999;
                }
            }
        }
    }

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy)| galaxies.iter().skip(i + 1).map(|other| (*galaxy, *other)))
        .map(|(galaxy, other)| {
            other.0.max(galaxy.0).sub(other.0.min(galaxy.0))
                + other.1.max(galaxy.1).sub(other.1.min(galaxy.1))
        })
        .sum::<usize>()
}
