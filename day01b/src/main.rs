mod solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_digits() {
        let expected = include_str!("../data/example_digits.txt")
            .split("\n")
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect::<Vec<i32>>();
        let converted = include_str!("../data/example.txt")
            .split("\n")
            .map(solution::convert_spelled_digits)
            .map(solution::get_calibration_value)
            .collect::<Vec<i32>>();
        assert_eq!(expected, converted);
    }

    #[test]
    fn example() {
        assert_eq!(
            solution::solution(include_str!("../data/example.txt")).to_string(),
            include_str!("../data/answer.txt")
        );
    }
}

fn main() {
    let start = std::time::Instant::now();
    let answer = solution::solution(include_str!("../data/input.txt"));
    let duration = start.elapsed();

    println!("Done in {:?}", duration);
    print!("{}", answer.to_string());
}
