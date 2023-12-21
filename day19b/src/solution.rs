// TODO: this solution is completely broken, I will return to it later.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Fate<'s> {
    Accepted,
    Rejected,
    Redirect(&'s str),
}

impl<'s> From<&'s str> for Fate<'s> {
    fn from(value: &'s str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Redirect(value),
        }
    }
}

#[derive(Clone, Copy)]
enum Rule<'s> {
    LessThan(char, usize, Fate<'s>),
    GreaterThan(char, usize, Fate<'s>),
    Default(Fate<'s>),
}

impl<'s> From<&'s str> for Rule<'s> {
    fn from(value: &'s str) -> Self {
        match value.find(['>', '<']) {
            Some(cmp_index) => {
                let fate_index = value.find(':').unwrap();
                let fate = &value[fate_index + 1..value.len()];
                let val = value[cmp_index + 1..fate_index].parse().unwrap();
                match &value[cmp_index..=cmp_index] {
                    "<" => Self::LessThan(value.chars().next().unwrap(), val, Fate::from(fate)),
                    ">" => Self::GreaterThan(value.chars().next().unwrap(), val, Fate::from(fate)),
                    _ => unreachable!(),
                }
            }
            None => Self::Default(Fate::from(value)),
        }
    }
}

type Combinations = HashMap<char, Option<(usize, usize)>>;

fn default_combinations() -> Combinations {
    Combinations::from([
        ('x', Some((1, 4000))),
        ('m', Some((1, 4000))),
        ('a', Some((1, 4000))),
        ('s', Some((1, 4000))),
    ])
}

fn empty_combinations() -> Combinations {
    Combinations::from([('x', None), ('m', None), ('a', None), ('s', None)])
}

fn narrow<'s>(
    combinations: &'s Combinations,
    rule: Rule<'s>,
) -> Option<((Combinations, Combinations), Fate<'s>)> {
    match rule {
        Rule::LessThan(kind, val, fate) => {
            if let Some(range) = combinations[&kind] {
                if range.0 < val {
                    if range.1 < val {
                        Some(((combinations.clone(), empty_combinations()), fate))
                    } else {
                        let mut new_combinations = combinations.clone();
                        let mut rest = combinations.clone();

                        new_combinations.insert(kind, Some((range.0, val - 1)));
                        rest.insert(kind, Some((val, range.1)));

                        Some(((new_combinations, rest), fate))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        Rule::GreaterThan(kind, val, fate) => {
            if let Some(range) = combinations[&kind] {
                if range.1 > val {
                    if range.0 > val {
                        Some(((combinations.clone(), empty_combinations()), fate))
                    } else {
                        let mut new_combinations = combinations.clone();
                        let mut rest = combinations.clone();

                        new_combinations.insert(kind, Some((val + 1, range.1)));
                        rest.insert(kind, Some((range.0, val)));

                        Some(((new_combinations, rest), fate))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        Rule::Default(fate) => Some(((combinations.clone(), empty_combinations()), fate)),
    }
}

fn calc_possibilities(combinations: &Combinations) -> usize {
    combinations
        .values()
        .filter_map(|o| o.map(|(l, h)| h - l))
        .sum()
}

fn possibilities(
    workflows: &HashMap<&str, Vec<Rule>>,
    workflow: &str,
    combinations: Combinations,
) -> usize {
    workflows[workflow]
        .iter()
        .filter_map(|rule| {
            if let Some(((new_combinations, rest), fate)) = narrow(&combinations, *rule) {
                Some(
                    match fate {
                        Fate::Accepted => calc_possibilities(&new_combinations),
                        Fate::Rejected => 0,
                        Fate::Redirect(key) => possibilities(workflows, key, new_combinations),
                    } + possibilities(workflows, workflow, rest),
                )
            } else {
                None
            }
        })
        .sum::<usize>()
}

pub fn solution(input: &str) -> impl ToString {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<&str, Vec<Rule>> = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line.split_once("{").unwrap();
            let rules = &rules[0..rules.len() - 1];

            (name, rules.split(",").map(Rule::from).collect())
        })
        .collect();

    possibilities(&workflows, "in", default_combinations())
}
