fn find_digit<I>(mut iter: I) -> i32
where
    I: Iterator<Item = char>,
{
    iter.find(|c| c.is_numeric())
        .unwrap()
        .to_string()
        .parse::<i32>()
        .unwrap()
}

fn get_calibration_value(line: &str) -> i32 {
    let first = find_digit(line.chars());
    let last = find_digit(line.chars().rev());
    (first * 10) + last
}

pub fn solution(input: &str) -> impl ToString {
    input.split("\n").map(get_calibration_value).sum::<i32>()
}
