pub fn solution(input: &str) -> impl ToString {
    input
        .split(",")
        .map(|step| {
            step.chars()
                .fold(0, |acc, b| ((acc + (b as u32)) * 17) % 256)
        })
        .sum::<u32>()
}
