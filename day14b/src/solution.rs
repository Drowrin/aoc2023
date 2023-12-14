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
    shift_up(grid);
    rotate(grid);
    shift_up(grid);
    rotate(grid);
    shift_up(grid);
    rotate(grid);
    shift_up(grid);
    rotate(grid);
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
        // I honestly don't know why, but any cycles discovered earlier than this don't work
        // even though a cycle should be a cycle, so the smallest cycle should work
        // TODO: revisit later?
        if states.len() > 568 {
            if let Some(start) = states.iter().position(|g| *g == grid) {
                return load(&states[1000000000 % (states.len() - start)]);
            }
        }
        states.push(grid.clone());
        cycle(&mut grid);
    }
}
