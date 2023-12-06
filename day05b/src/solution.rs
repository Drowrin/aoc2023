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

                    (
                        src_start,
                        src_start + length - 1,
                        dest_start,
                        dest_start + length - 1,
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    seed_line
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let (s, l) = chunk.collect_tuple().unwrap();
            vec![(s, s + l - 1)]
        })
        .flat_map(|initial_ranges| {
            maps.iter().fold(initial_ranges, |ranges, map| {
                let mut remaining_ranges = ranges;
                let mut processed_ranges = vec![];

                for (src_start, src_end, dst_start, dst_end) in map {
                    let mut new_remaining = vec![];

                    for (range_start, range_end) in remaining_ranges {
                        if range_start > *src_end {
                            // start is after map, entire range gets reprocessed
                            new_remaining.push((range_start, range_end));
                            continue;
                        }
                        if range_end < *src_start {
                            // end is before map, entire range gets reprocessed
                            new_remaining.push((range_start, range_end));
                            continue;
                        }
                        if range_start >= *src_start {
                            // start is within map
                            if range_end <= *src_end {
                                // entire range is within map
                                // entire range gets mapped
                                processed_ranges.push((
                                    *dst_start + range_start - *src_start,
                                    *dst_start + range_end - *src_start,
                                ));
                            } else {
                                // end is after map
                                // start of range until end of map gets mapped
                                processed_ranges
                                    .push((*dst_start + range_start - *src_start, *dst_end));
                                // end of map until end of range goes back into unmapped ranges
                                new_remaining.push((*src_end + 1, range_end));
                            }
                        } else {
                            // end is after map start
                            if range_end > *src_end {
                                // range covers entire map
                                // map range gets mapped
                                processed_ranges.push((*dst_start, *dst_end));
                                // end of map until end of range goes back into unmapped ranges
                                new_remaining.push((*src_end + 1, range_end));
                                // start of range until start of map goes back into unmapped ranges
                                new_remaining.push((range_start, *src_start - 1));
                            } else {
                                // end is within map
                                // start of map until end of range gets mapped
                                processed_ranges
                                    .push((*dst_start, *dst_start + range_end - *src_start));
                                // start of range until start of map goes back into unmapped ranges
                                new_remaining.push((range_start, *src_start - 1));
                            }
                        }
                    }

                    remaining_ranges = new_remaining;
                }

                processed_ranges.extend(remaining_ranges);

                processed_ranges
            })
        })
        .min_by_key(|l| l.0)
        .unwrap()
        .0
}
