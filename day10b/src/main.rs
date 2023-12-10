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
    #[test]
    fn example4() {
        assert_eq!(
            solution::solution(include_str!("../data/example4.txt")).to_string(),
            include_str!("../data/answer4.txt")
        );
    }
    #[test]
    fn example5() {
        assert_eq!(
            solution::solution(include_str!("../data/example5.txt")).to_string(),
            include_str!("../data/answer5.txt")
        );
    }
    #[test]
    fn example6() {
        assert_eq!(
            solution::solution(include_str!("../data/example6.txt")).to_string(),
            include_str!("../data/answer6.txt")
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
