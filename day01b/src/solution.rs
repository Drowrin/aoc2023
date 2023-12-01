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

pub fn get_calibration_value(line: String) -> i32 {
    let first = find_digit(line.chars());
    let last = find_digit(line.chars().rev());
    (first * 10) + last
}

// this is not particularly fast, but it works
pub fn convert_spelled_digits(line: &str) -> String {
    // these extra characters ensure that overlapping numbers are counted
    line.replace("one", "o1e")
        .replace("two", "t2")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(convert_spelled_digits)
        .map(get_calibration_value)
        .sum::<i32>()
}
