use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn opposite(&self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleKind<'s> {
    FlipFlop { state: Pulse },
    Conjunction { inputs: HashMap<&'s str, Pulse> },
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module<'s> {
    kind: ModuleKind<'s>,
    name: &'s str,
    dest: Vec<&'s str>,
}

impl<'s> From<&'s str> for Module<'s> {
    fn from(line: &'s str) -> Self {
        let (module, dest) = line.split_once(" -> ").unwrap();
        let dest = dest.split(", ").collect();

        if module == "broadcaster" {
            Module {
                kind: ModuleKind::Broadcast,
                name: module,
                dest,
            }
        } else {
            let (kind, name) = module.split_at(1);
            Module {
                kind: match kind {
                    "%" => ModuleKind::FlipFlop { state: Pulse::Low },
                    "&" => ModuleKind::Conjunction {
                        inputs: HashMap::new(),
                    },
                    _ => unreachable!(),
                },
                name,
                dest,
            }
        }
    }
}

impl<'s> Module<'s> {
    fn pulse(&mut self, p: Pulse, from: &'s str) -> Vec<(&'s str, Pulse, &'s str)> {
        let new_pulse: Option<Pulse> = match &mut self.kind {
            ModuleKind::FlipFlop { state: on } => match p {
                Pulse::Low => {
                    *on = on.opposite();
                    Some(*on)
                }
                Pulse::High => None,
            },
            ModuleKind::Conjunction { inputs } => {
                inputs.insert(from, p);
                if inputs.values().all(|mem| *mem == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleKind::Broadcast => Some(p),
        };

        if let Some(new_pulse) = new_pulse {
            self.dest
                .iter()
                .map(|dest| (*dest, new_pulse, self.name))
                .collect()
        } else {
            vec![]
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(Module::from)
        .map(|module| (module.name, module))
        .collect();

    let keys = modules.keys().map(|s| *s).collect::<Vec<_>>().clone();
    let dummy_modules = modules.clone();

    for k in keys.iter() {
        match &mut modules.get_mut(*k).unwrap().kind {
            ModuleKind::Conjunction { inputs } => {
                for pk in keys.iter() {
                    if dummy_modules.get(*pk).unwrap().dest.contains(k) {
                        inputs.insert(pk, Pulse::Low);
                    }
                }
            }
            _ => (),
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut pending_pulses = vec![("broadcaster", Pulse::Low, "button")];

        while pending_pulses.len() > 0 {
            low_pulses += pending_pulses
                .iter()
                .filter(|(_, p, _)| *p == Pulse::Low)
                .count();
            high_pulses += pending_pulses
                .iter()
                .filter(|(_, p, _)| *p == Pulse::High)
                .count();

            pending_pulses = pending_pulses
                .iter()
                .flat_map(|(t, p, f)| {
                    if let Some(module) = modules.get_mut(t) {
                        module.pulse(*p, f)
                    } else {
                        vec![]
                    }
                })
                .collect();
        }
    }

    low_pulses * high_pulses
}
