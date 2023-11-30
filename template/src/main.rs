mod solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            solution::solution(include_str!("../data/example.txt")),
            include_str!("../data/answer.txt")
        );
    }
}

fn main() {
    print!("{}", solution::solution(include_str!("../data/input.txt")));
}
