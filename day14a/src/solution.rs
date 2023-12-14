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

fn load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.into_iter().filter(|c| **c == 'O').count() * (i + 1))
        .sum()
}

pub fn solution(input: &str) -> impl ToString {
    let mut grid = input.lines().map(|line| line.chars().collect()).collect();

    shift_up(&mut grid);

    load(&grid)
}
