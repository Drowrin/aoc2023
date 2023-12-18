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
}

fn main() {
    let start = std::time::Instant::now();
    let answer = solution::solution(include_str!("../data/input.txt"));
    let duration = start.elapsed();

    println!("Done in {:?}", duration);
    print!("{}", answer.to_string());
}
