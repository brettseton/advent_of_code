use std::{
    collections::{HashMap, VecDeque},
    fs,
    str::FromStr,
};

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 : {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 : {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 : {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let mut machine = Machine::new(&input);
    machine.get_output()
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let mut machine = Machine::new(&input);
    machine.get_output_lcm()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ModuleId(usize);

#[derive(Clone, Debug)]
struct Signal {
    to: ModuleId,
    from: ModuleId,
    state: bool,
}

#[derive(Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop {
        state: bool,
    },
    Conjunction {
        remembered_pulses: HashMap<ModuleId, bool>,
    },
    Sink,
}

#[derive(Clone)]
struct Module {
    id: ModuleId,
    module_type: ModuleType,
    destinations: Vec<ModuleId>,
}

struct Machine {
    modules: Vec<Module>,
    label_to_id: HashMap<String, ModuleId>,
    broadcaster_id: ModuleId,
}

impl Machine {
    pub fn new(str: &str) -> Machine {
        Machine::from_str(str).expect("Failed to parse machine")
    }

    pub fn get_output(&mut self) -> usize {
        let mut high = 0;
        let mut low = 0;
        let mut queue: VecDeque<Signal> = VecDeque::with_capacity(1024);
        let button_id = ModuleId(usize::MAX);

        for _ in 0..1000 {
            low += 1;
            queue.push_back(Signal {
                to: self.broadcaster_id,
                from: button_id,
                state: false,
            });

            while let Some(signal) = queue.pop_front() {
                if signal.to.0 >= self.modules.len() {
                    continue;
                }

                let module = &mut self.modules[signal.to.0];
                let next_state = match &mut module.module_type {
                    ModuleType::Broadcaster => Some(signal.state),
                    ModuleType::FlipFlop { state } => {
                        if signal.state {
                            None
                        } else {
                            *state = !*state;
                            Some(*state)
                        }
                    }
                    ModuleType::Conjunction { remembered_pulses } => {
                        remembered_pulses.insert(signal.from, signal.state);
                        Some(!remembered_pulses.values().all(|&v| v))
                    }
                    ModuleType::Sink => None,
                };

                if let Some(state) = next_state {
                    for &dest in &module.destinations {
                        if state {
                            high += 1;
                        } else {
                            low += 1;
                        }
                        queue.push_back(Signal {
                            to: dest,
                            from: module.id,
                            state,
                        });
                    }
                }
            }
        }

        high * low
    }

    pub fn get_output_lcm(&mut self) -> usize {
        let rx_id = match self.label_to_id.get("rx") {
            Some(&id) => id,
            None => return 0,
        };

        let rx_parent_idx = self
            .modules
            .iter()
            .position(|m| m.destinations.contains(&rx_id));
        let rx_parent_id = match rx_parent_idx {
            Some(idx) => ModuleId(idx),
            None => return 0,
        };

        let mut visited: HashMap<ModuleId, usize> = HashMap::new();
        if let ModuleType::Conjunction { remembered_pulses } =
            &self.modules[rx_parent_id.0].module_type
        {
            for &input_id in remembered_pulses.keys() {
                visited.insert(input_id, 0);
            }
        } else {
            return 0;
        }

        let mut queue: VecDeque<Signal> = VecDeque::with_capacity(1024);
        let button_id = ModuleId(usize::MAX);

        for i in 1..=usize::MAX {
            queue.push_back(Signal {
                to: self.broadcaster_id,
                from: button_id,
                state: false,
            });

            while let Some(signal) = queue.pop_front() {
                if signal.to == rx_parent_id && signal.state {
                    if let Some(val) = visited.get_mut(&signal.from) {
                        if *val == 0 {
                            *val = i;
                        }
                    }

                    if visited.values().all(|&v| v != 0) {
                        return visited
                            .values()
                            .fold(1, |acc, &v| num::integer::lcm(acc, v));
                    }
                }

                if signal.to.0 >= self.modules.len() {
                    continue;
                }

                let module = &mut self.modules[signal.to.0];
                let next_state = match &mut module.module_type {
                    ModuleType::Broadcaster => Some(signal.state),
                    ModuleType::FlipFlop { state } => {
                        if signal.state {
                            None
                        } else {
                            *state = !*state;
                            Some(*state)
                        }
                    }
                    ModuleType::Conjunction { remembered_pulses } => {
                        remembered_pulses.insert(signal.from, signal.state);
                        Some(!remembered_pulses.values().all(|&v| v))
                    }
                    ModuleType::Sink => None,
                };

                if let Some(state) = next_state {
                    for &dest in &module.destinations {
                        queue.push_back(Signal {
                            to: dest,
                            from: module.id,
                            state,
                        });
                    }
                }
            }
        }
        0
    }
}

#[derive(Debug)]
struct MachineError;

impl FromStr for Machine {
    type Err = MachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut label_to_id = HashMap::new();
        let mut raw_lines = Vec::new();

        for line in s.lines() {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(" -> ").collect();
            let mut label = parts[0];
            let prefix = if label.starts_with(['%', '&']) {
                let p = label.chars().next().unwrap();
                label = &label[1..];
                Some(p)
            } else {
                None
            };

            let next_id = ModuleId(label_to_id.len());
            let id = *label_to_id.entry(label.to_string()).or_insert(next_id);
            raw_lines.push((id, label.to_string(), prefix, parts[1]));
        }

        let mut modules_map: HashMap<ModuleId, Module> = HashMap::new();
        for (id, _, prefix, dest_str) in raw_lines {
            let mut destinations = Vec::new();
            for d in dest_str.split(", ") {
                let next_id = ModuleId(label_to_id.len());
                let did = *label_to_id.entry(d.to_string()).or_insert(next_id);
                destinations.push(did);
            }

            let module_type = match prefix {
                Some('%') => ModuleType::FlipFlop { state: false },
                Some('&') => ModuleType::Conjunction {
                    remembered_pulses: HashMap::new(),
                },
                _ => ModuleType::Broadcaster,
            };

            modules_map.insert(
                id,
                Module {
                    id,
                    module_type,
                    destinations,
                },
            );
        }

        let mut modules = Vec::with_capacity(label_to_id.len());
        for i in 0..label_to_id.len() {
            let id = ModuleId(i);
            if let Some(m) = modules_map.remove(&id) {
                modules.push(m);
            } else {
                modules.push(Module {
                    id,
                    module_type: ModuleType::Sink,
                    destinations: vec![],
                });
            }
        }

        let module_count = modules.len();
        for i in 0..module_count {
            let mut inputs = Vec::new();
            let current_id = modules[i].id;
            for sender in &modules {
                if sender.destinations.contains(&current_id) {
                    inputs.push(sender.id);
                }
            }

            if let ModuleType::Conjunction { remembered_pulses } = &mut modules[i].module_type {
                for input_id in inputs {
                    remembered_pulses.insert(input_id, false);
                }
            }
        }

        let broadcaster_id = *label_to_id.get("broadcaster").ok_or(MachineError)?;

        Ok(Machine {
            modules,
            label_to_id,
            broadcaster_id,
        })
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 32000000);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 929810733);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 0);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 231657829136023);
}
