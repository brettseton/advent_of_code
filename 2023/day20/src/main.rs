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
    return machine.get_output();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let mut machine = Machine::new(&input);
    return machine.get_output_lcm();
}

#[allow(dead_code)]
trait IModule {
    fn send_messages(&self);
    fn receive_message(&mut self, signal: Signal) -> Vec<Signal>;
    fn get_destinations(&self) -> Vec<String>;
    fn get_label(&self) -> String;
}

struct Machine {
    modules: Vec<IModuleType>,
}

impl Machine {
    pub fn new(str: &str) -> Machine {
        return Machine::from_str(str).expect("");
    }

    pub fn get_output(&mut self) -> usize {
        let mut broadcaster = self
            .modules
            .iter()
            .filter(|x| x.get_label() == "broadcaster")
            .nth(0)
            .expect("broadcaster is required")
            .clone();
        let mut high = 0;
        let mut low = 0;
        for _i in 1..=1000 {
            let mut queue: VecDeque<Signal> = broadcaster
                .receive_message(Signal {
                    to: "broadcaster".to_string(),
                    from: "main".to_string(),
                    state: false,
                })
                .into();
            low += 1;
            while let Some(s) = queue.pop_front() {
                if s.state {
                    high += 1;
                } else {
                    low += 1;
                }

                let module = match self.modules.iter_mut().find(|x| x.get_label() == s.to) {
                    Some(m) => m,
                    None => {
                        continue;
                    }
                };

                let signals = module.receive_message(s);
                for signal in signals.iter() {
                    queue.push_back(signal.clone());
                }
            }
        }

        return high * low;
    }

    pub fn get_output_lcm(&mut self) -> usize {
        let mut broadcaster = self
            .modules
            .iter()
            .filter(|x| x.get_label() == "broadcaster")
            .nth(0)
            .expect("broadcaster is required")
            .clone();

        let Some(rx_parent) = self
            .modules
            .iter()
            .filter(|x| x.get_destinations().iter().any(|x| x == "rx"))
            .nth(0)
        else {
            return 0;
        };

        let Some(rx_conjunction) = rx_parent.as_conjunction() else {
            panic!("")
        };
        let mut visited: HashMap<String, usize> = rx_conjunction
            .remembered_pulses
            .keys()
            .map(|k| (k.clone(), 0))
            .collect();
        for i in 1..=usize::MAX {
            let mut queue: VecDeque<Signal> = broadcaster
                .receive_message(Signal {
                    to: "broadcaster".to_string(),
                    from: "main".to_string(),
                    state: false,
                })
                .into();
            while let Some(s) = queue.pop_front() {
                if s.to == rx_conjunction.label && s.state {
                    if let Some(val) = visited.get_mut(&s.from) {
                        *val = i;
                    }
                }

                if visited.iter().all(|(_k, &v)| v != 0) {
                    let mut lcm = 1;
                    for v in visited {
                        lcm = num::integer::lcm(lcm, v.1);
                    }
                    return lcm;
                }

                let module = match self.modules.iter_mut().find(|x| x.get_label() == s.to) {
                    Some(m) => m,
                    None => {
                        continue;
                    }
                };

                let signals = module.receive_message(s);
                for signal in signals.iter() {
                    queue.push_back(signal.clone());
                }
            }
        }

        return 0;
    }
}

#[derive(Debug)]
struct MachineError;

impl FromStr for Machine {
    type Err = MachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules: Vec<IModuleType> = s.lines().map(IModuleFactory::new_module).collect();

        let module_lookup = modules.clone();

        for module in modules.iter_mut() {
            match module {
                IModuleType::Broadcaster(_) => (),
                IModuleType::FlipFlop(_) => (),
                IModuleType::Conjunction(c) => {
                    let labels: Vec<String> = module_lookup
                        .iter()
                        .filter(|&x| x.get_destinations().iter().any(|d| d.eq(&c.label)))
                        .map(|x| x.get_label())
                        .collect();
                    for label in labels {
                        c.remembered_pulses.insert(label, false);
                    }
                }
            }
        }

        return Ok(Machine { modules });
    }
}

#[derive(Clone)]
enum IModuleType {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl IModuleType {
    pub fn as_conjunction(&self) -> Option<Conjunction> {
        return match self {
            IModuleType::Broadcaster(_b) => None,
            IModuleType::FlipFlop(_f) => None,
            IModuleType::Conjunction(c) => Some(c.clone()),
        };
    }
}

#[derive(Clone)]
struct Signal {
    to: String,
    from: String,
    state: bool,
}

#[derive(Clone)]
struct FlipFlop {
    label: String,
    state: bool,
    destinations: Vec<String>,
}

impl IModule for IModuleType {
    fn send_messages(&self) {
        match self {
            IModuleType::Broadcaster(b) => b.send_messages(),
            IModuleType::FlipFlop(f) => f.send_messages(),
            IModuleType::Conjunction(c) => c.send_messages(),
        }
    }

    fn receive_message(&mut self, signal: Signal) -> Vec<Signal> {
        return match self {
            IModuleType::Broadcaster(b) => b.receive_message(signal),
            IModuleType::FlipFlop(f) => f.receive_message(signal),
            IModuleType::Conjunction(c) => c.receive_message(signal),
        };
    }

    fn get_destinations(&self) -> Vec<String> {
        return match self {
            IModuleType::Broadcaster(b) => b.get_destinations(),
            IModuleType::FlipFlop(f) => f.get_destinations(),
            IModuleType::Conjunction(c) => c.get_destinations(),
        };
    }

    fn get_label(&self) -> String {
        return match self {
            IModuleType::Broadcaster(b) => b.get_label(),
            IModuleType::FlipFlop(f) => f.get_label(),
            IModuleType::Conjunction(c) => c.get_label(),
        };
    }
}

impl IModule for FlipFlop {
    fn send_messages(&self) {
        println!("I am Flip Flop {}", self.label);
    }

    fn receive_message(&mut self, signal: Signal) -> Vec<Signal> {
        if signal.state {
            return vec![];
        }

        self.state = !self.state;
        return self
            .destinations
            .iter()
            .map(|x| Signal {
                to: x.to_string(),
                from: self.label.to_string(),
                state: self.state,
            })
            .collect();
    }

    fn get_destinations(&self) -> Vec<String> {
        return self.destinations.clone();
    }

    fn get_label(&self) -> String {
        return self.label.clone();
    }
}

#[derive(Clone)]
struct Broadcaster {
    label: String,
    destinations: Vec<String>,
}

impl IModule for Broadcaster {
    fn send_messages(&self) {
        println!("I am Broadcaster {}", self.label);
    }

    fn receive_message(&mut self, signal: Signal) -> Vec<Signal> {
        return self
            .destinations
            .iter()
            .map(|x| Signal {
                to: x.to_string(),
                from: self.label.to_string(),
                state: signal.state,
            })
            .collect();
    }

    fn get_destinations(&self) -> Vec<String> {
        return self.destinations.clone();
    }

    fn get_label(&self) -> String {
        return self.label.clone();
    }
}

#[derive(Clone)]
struct Conjunction {
    label: String,
    destinations: Vec<String>,
    remembered_pulses: HashMap<String, bool>,
}

impl IModule for Conjunction {
    fn send_messages(&self) {
        println!("I am Conjunction {}", self.label);
    }

    fn receive_message(&mut self, signal: Signal) -> Vec<Signal> {
        // set state
        if let Some(val) = self.remembered_pulses.get_mut(&signal.from) {
            *val = signal.state;
        }

        let state = !self.remembered_pulses.iter().all(|(_k, &v)| v);

        return self
            .destinations
            .iter()
            .map(|x| Signal {
                to: x.to_string(),
                from: self.label.to_string(),
                state,
            })
            .collect();
    }

    fn get_destinations(&self) -> Vec<String> {
        return self.destinations.clone();
    }

    fn get_label(&self) -> String {
        return self.label.clone();
    }
}

struct IModuleFactory {}

impl IModuleFactory {
    pub fn new_module(str: &str) -> IModuleType {
        let [label, destinations] =
            &str.split(" -> ").map(String::from).collect::<Vec<String>>()[..]
        else {
            panic!()
        };

        let module: IModuleType = match str.chars().nth(0) {
            Some('b') => IModuleType::Broadcaster(Broadcaster {
                label: label[..].to_string(),
                destinations: destinations.split(", ").map(String::from).collect(),
            }),
            Some('%') => IModuleType::FlipFlop(FlipFlop {
                label: label[1..].to_string(),
                state: false,
                destinations: destinations.split(", ").map(String::from).collect(),
            }),
            Some('&') => IModuleType::Conjunction(Conjunction {
                label: label[1..].to_string(),
                destinations: destinations.split(", ").map(String::from).collect(),
                remembered_pulses: HashMap::new(),
            }),
            _ => panic!("not a valid type"),
        };

        return module;
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
