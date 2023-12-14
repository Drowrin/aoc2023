fn horizontal_reflection(i: usize, max_c: usize, pattern: &Vec<Vec<char>>) -> usize {
    let min = if i > max_c / 2 { i * 2 - max_c - 1 } else { 0 };
    let max = if i <= max_c / 2 { i * 2 - 1 } else { max_c };

    pattern
        .iter()
        .map(|row| {
            (i..=max)
                .rev()
                .zip(min..i)
                .map(|(right, left)| if row[left] == row[right] { 0 } else { 1 })
                .sum::<usize>()
        })
        .sum()
}

fn vertical_reflection(i: usize, max_r: usize, max_c: usize, pattern: &Vec<Vec<char>>) -> usize {
    let min = if i > max_r / 2 { i * 2 - max_r - 1 } else { 0 };
    let max = if i <= max_r / 2 { i * 2 - 1 } else { max_r };

    (0..=max_c)
        .map(|c| {
            (i..=max)
                .rev()
                .zip(min..i)
                .map(|(bot, top)| {
                    if pattern[top][c] == pattern[bot][c] {
                        0
                    } else {
                        1
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .split("\n")
                .map(|line| line.chars().collect())
                .collect()
        })
        .map(|pattern: Vec<Vec<char>>| {
            let max_c = pattern[0].len() - 1;
            let max_r = pattern.len() - 1;

            for i in 1..=max_c {
                if horizontal_reflection(i, max_c, &pattern) == 1 {
                    return i;
                }
            }

            for i in 1..=max_r {
                if vertical_reflection(i, max_r, max_c, &pattern) == 1 {
                    return i * 100;
                }
            }

            return 0;
        })
        .sum::<usize>()
}
