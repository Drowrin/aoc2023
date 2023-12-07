use itertools::Itertools;

pub fn solution(input: &str) -> impl ToString {
    let (time_limits, distance_records) = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|token| token.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    time_limits
        .into_iter()
        .zip(distance_records)
        .map(|(time_limit, distance_record)| {
            let y = time_limit as f64;
            let width = ((time_limit.pow(2) - 4 * distance_record) as f64).sqrt();
            let low_root = ((y - width) / 2.0).floor();
            let high_root = ((y + width) / 2.0).ceil();

            (high_root - low_root) - 1.0
        })
        .product::<f64>()
}
