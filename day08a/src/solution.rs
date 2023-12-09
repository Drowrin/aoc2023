use std::collections::HashMap;

pub fn solution(input: &str) -> impl ToString {
    let mut lines = input.split("\n");
    let instructions: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Unrecognized instruction {c}"),
        })
        .collect();

    let nodes: HashMap<&str, [&str; 2]> = lines
        .skip(1)
        .map(|line| (&line[0..3], [&line[7..10], &line[12..15]]))
        .collect();

    let mut steps = 0;
    let mut node = "AAA";
    let mut iter = instructions.into_iter().cycle();
    loop {
        steps += 1;
        let instruction = iter.next().unwrap();
        node = nodes[node][instruction];

        if node == "ZZZ" {
            return steps;
        }
    }
}
