fn horizontal_reflection(i: usize, max_c: usize, pattern: &Vec<Vec<char>>) -> bool {
    let min = if i > max_c / 2 { i * 2 - max_c - 1 } else { 0 };
    let max = if i <= max_c / 2 { i * 2 - 1 } else { max_c };

    pattern.iter().all(|row| {
        (i..=max)
            .rev()
            .zip(min..i)
            .all(|(right, left)| row[left] == row[right])
    })
}

fn vertical_reflection(i: usize, max_r: usize, max_c: usize, pattern: &Vec<Vec<char>>) -> bool {
    let min = if i > max_r / 2 { i * 2 - max_r - 1 } else { 0 };
    let max = if i <= max_r / 2 { i * 2 - 1 } else { max_r };

    (0..=max_c).all(|c| {
        (i..=max)
            .rev()
            .zip(min..i)
            .all(|(bot, top)| pattern[top][c] == pattern[bot][c])
    })
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
                if horizontal_reflection(i, max_c, &pattern) {
                    return i;
                }
            }

            for i in 1..=max_r {
                if vertical_reflection(i, max_r, max_c, &pattern) {
                    return i * 100;
                }
            }

            return 0;
        })
        .sum::<usize>()
}
