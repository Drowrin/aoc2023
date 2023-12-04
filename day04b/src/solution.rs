pub fn solution(input: &str) -> impl ToString {
    let card_wins = input
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
        .map(|(winners, ours)| {
            ours.iter()
                .filter(|o| winners.contains(o))
                .map(|o| o.to_owned())
                .collect::<Vec<_>>()
        })
        .map(|our_winners| our_winners.len())
        .collect::<Vec<_>>();

    let length = card_wins.len();
    let mut card_counts = vec![1; length];

    for i in 0..length {
        let start = i + 1;
        let end = start.saturating_add(card_wins[i]).min(length);

        for j in start..end {
            card_counts[j] += card_counts[i];
        }
    }

    card_counts.iter().sum::<u32>()
}
