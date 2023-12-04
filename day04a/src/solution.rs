pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(|line| line.split_at(line.find("|").unwrap()))
        .map(|(left, right)| {
            (
                left.split(" ")
                    .skip(2)
                    .flat_map(|s| s.parse::<u32>())
                    .collect::<Vec<_>>(),
                right
                    .split(" ")
                    .flat_map(|s| s.parse::<u32>())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(winners, ours)| ours.iter().filter(|o| winners.contains(o)).count() as u32)
        .map(|count| if count == 0 { 0 } else { 2_u32.pow(count - 1) })
        .sum::<u32>()
}
