#[derive(Clone, Copy)]
struct Lens<'s> {
    label: &'s str,
    length: usize,
}

enum Step<'s> {
    Remove(usize, &'s str),
    Set(usize, &'s str, usize),
}

fn parse<'s>(step: &'s str) -> Step<'s> {
    let mut hash = 0;
    let mut iter = step.chars();
    let mut label_length = 0;
    loop {
        let c = iter.next().unwrap();
        if c.is_alphabetic() {
            hash = ((hash + (c as u32)) * 17) % 256;
            label_length += 1;
        } else {
            let label = &step[0..label_length];
            return match c {
                '-' => Step::Remove(hash as usize, label),
                '=' => Step::Set(hash as usize, label, iter.next().unwrap() as usize - 48),
                _ => unreachable!(),
            };
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for step in input.split(",").map(parse) {
        match step {
            Step::Remove(hash, label) => {
                if let Some(index) = boxes[hash].iter().position(|s| s.label == label) {
                    boxes[hash].remove(index);
                }
            }
            Step::Set(hash, label, length) => {
                match boxes[hash].iter().position(|s| s.label == label) {
                    Some(index) => boxes[hash][index].length = length,
                    None => boxes[hash].push(Lens { label, length }),
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(ii, s)| (1 + i) * (1 + ii) * s.length)
                .sum::<usize>()
        })
        .sum::<usize>()
}
