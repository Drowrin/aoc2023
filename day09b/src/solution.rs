use itertools::Itertools;

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|history| {
            let mut histories = vec![history];

            while !histories.last().unwrap().iter().all(|x| *x == 0) {
                histories.push(
                    histories
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(left, right)| right - left)
                        .collect(),
                );
            }

            histories
                .iter()
                .rev()
                .fold(0, |acc, history| history.first().unwrap() - acc)
        })
        .sum::<i64>()
}
