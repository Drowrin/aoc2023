use itertools::Itertools;
use rayon::prelude::*;

const GROUP_CACHE_SIZE: usize = 31;
const SPRING_CACHE_SIZE: usize = 105;
type Cache = [[usize; GROUP_CACHE_SIZE]; SPRING_CACHE_SIZE];

fn arrangements(cache: &mut Cache, springs: &[char], groups: &[usize], last_broken: bool) -> usize {
    if !last_broken {
        match springs {
            ['.', rest @ ..] => arrangements(cache, rest, groups, false),
            ['#', ..] => arrangements(cache, springs, groups, true),
            ['?', rest @ ..] => {
                // combine possible arrangements of both cases
                arrangements(cache, rest, groups, false)
                    + arrangements(cache, springs, groups, true)
            }
            [] if groups.is_empty() => 1,
            _ => 0,
        }
    } else {
        match cache[springs.len()][groups.len()] {
            usize::MAX => match groups {
                [] => 0,

                [first, rest @ ..] => match springs.len() {
                    l if l < *first => 0,
                    l if l == *first => {
                        if rest.len() == 0 && !springs[0..*first].iter().any(|c| *c == '.') {
                            1
                        } else {
                            0
                        }
                    }
                    l => {
                        if springs[*first] == '#' {
                            0
                        } else {
                            if springs[0..*first].iter().any(|c| *c == '.') {
                                0
                            } else {
                                let result =
                                    arrangements(cache, &springs[(first + 1)..], rest, false);
                                cache[l][groups.len()] = result;
                                result
                            }
                        }
                    }
                },
            },
            result => result,
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    input
        .lines()
        .collect_vec()
        .par_iter()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(springs, groups)| {
            (
                springs.chars().collect_vec(),
                groups.split(',').map(|n| n.parse().unwrap()).collect_vec(),
            )
        })
        .map(|(springs, groups)| {
            let mut cache = [[usize::MAX; GROUP_CACHE_SIZE]; SPRING_CACHE_SIZE];
            arrangements(&mut cache, springs.as_slice(), groups.as_slice(), false)
        })
        .sum::<usize>()
}
