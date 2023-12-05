use itertools::Itertools;

pub fn solution(input: &str) -> impl ToString {
    let (seed_line, map_lines) = input.split_at(input.find("\n\n").unwrap());

    let maps = map_lines
        .split("\n\n")
        .skip(1)
        .map(|section| {
            section
                .split("\n")
                .skip(1)
                .map(|line| {
                    let (dest_start, src_start, length) = line
                        .split(" ")
                        .map(|token| token.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    (src_start, dest_start, length)
                })
                .collect_vec()
        })
        .collect_vec();

    seed_line
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .map(|initial_seed| {
            maps.iter().fold(initial_seed, |seed, map| {
                map.into_iter()
                    .find_map(|(src, dst, len)| {
                        if seed < *src || seed >= (src + len) {
                            None
                        } else {
                            Some(dst + seed - src)
                        }
                    })
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}
