use itertools::Itertools;

pub fn solution(input: &str) -> impl ToString {
    let (time_limit, distance_record) = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let y = time_limit as f64;
    let width = ((time_limit.pow(2) - 4 * distance_record) as f64).sqrt();
    let low_root = ((y - width) / 2.0).floor();
    let high_root = ((y + width) / 2.0).ceil();

    (high_root - low_root) - 1.0
}
