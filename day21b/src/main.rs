mod solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            solution::solution(include_str!("../data/example.txt")).to_string(),
            include_str!("../data/answer.txt")
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            solution::solution(include_str!("../data/example2.txt")).to_string(),
            include_str!("../data/answer2.txt")
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            solution::solution(include_str!("../data/example3.txt")).to_string(),
            include_str!("../data/answer3.txt")
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
