fn shift_up(grid: &mut Vec<Vec<char>>) {
    for r in 1..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'O' {
                let mut new_row = r;
                grid[r][c] = '.';
                while new_row != 0 && grid[new_row - 1][c] == '.' {
                    new_row -= 1;
                }
                grid[new_row][c] = 'O';
            }
        }
    }
}

fn rotate(grid: &mut Vec<Vec<char>>) {
    *grid = (0..grid[0].len())
        .map(|c| (0..grid.len()).rev().map(|r| grid[r][c]).collect())
        .collect();
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    shift_up(grid); // north
    rotate(grid);
    shift_up(grid); // west
    rotate(grid);
    shift_up(grid); // south
    rotate(grid);
    shift_up(grid); // east
    rotate(grid); // put north back up top
}

fn load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.into_iter().filter(|c| **c == 'O').count() * (i + 1))
        .sum()
}

pub fn solution(input: &str) -> impl ToString {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut states = vec![];

    loop {
        if let Some(start) = states.iter().position(|g| *g == grid) {
            return load(&states[(1000000000 - start) % (states.len() - start) + start]);
        }
        states.push(grid.clone());
        cycle(&mut grid);
    }
}
