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

enum Rule<'s> {
    LessThan(&'s str, usize, Fate<'s>),
    GreaterThan(&'s str, usize, Fate<'s>),
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
                    "<" => Self::LessThan(&value[0..cmp_index], val, Fate::from(fate)),
                    ">" => Self::GreaterThan(&value[0..cmp_index], val, Fate::from(fate)),
                    _ => unreachable!(),
                }
            }
            None => Self::Default(Fate::from(value)),
        }
    }
}

type Part<'s> = HashMap<&'s str, usize>;

fn parse_part<'s>(input: &'s str) -> Part<'s> {
    let input = &input[1..input.len() - 1];
    input
        .split(",")
        .map(|rule| {
            let (cat, val) = rule.split_once("=").unwrap();
            (cat, val.parse().unwrap())
        })
        .collect()
}

pub fn solution(input: &str) -> impl ToString {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Vec<Rule>> = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line.split_once("{").unwrap();
            let rules = &rules[0..rules.len() - 1];

            (name.to_string(), rules.split(",").map(Rule::from).collect())
        })
        .collect();

    parts
        .lines()
        .map(parse_part)
        .filter(|part| {
            let mut fate = Fate::Redirect("in");

            while let Fate::Redirect(name) = fate {
                fate = workflows[name]
                    .iter()
                    .find_map(|rule| match rule {
                        Rule::LessThan(cat, val, f) if part[cat] < *val => Some(*f),
                        Rule::GreaterThan(cat, val, f) if part[cat] > *val => Some(*f),
                        Rule::Default(f) => Some(*f),
                        _ => None,
                    })
                    .unwrap();
            }

            return fate == Fate::Accepted;
        })
        .map(|part| part.values().sum::<usize>())
        .sum::<usize>()
}
