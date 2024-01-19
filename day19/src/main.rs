use std::{fs, str::FromStr};

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
    let machine = Machine::new(&input);
    return machine.get_accepted_sum();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let machine = Machine::new(&input);
    return machine.get_accepted_sum();
}

struct Machine{
   workflows: Vec<Workflow>,
   input_conditions: Vec<Vec<(String, usize)>>
}

impl Machine {
    pub fn new(str: &str) -> Machine {
        return Machine::from_str(str).expect("");
    }
    
    pub fn get_accepted_sum(&self) -> usize {
        let mut sum = 0;

        for input_condition in self.input_conditions.iter() {
            if self.is_accepted(input_condition) {
                sum += input_condition.iter().map(|x| x.1).sum::<usize>();
            }
        }
        
        return sum;
    }

    pub fn is_accepted(&self, input_condition: &Vec<(String, usize)>) -> bool {
        let mut current_label = "in";

        while let Some(workflow) = self.workflows.iter().find(|&x| x.label == current_label) {
            for rule in workflow.rules.iter() {

                if rule.1.is_none(){
                    current_label = rule.0.as_str();
                    break;
                }
                
                if let [label_str, amount_str] = &rule.0.split("<").map(String::from).collect::<Vec<String>>()[..] {
                    let input_value = match input_condition.iter().find(|&x| x.0.eq(label_str)) {
                        Some(x) => x.1,
                        None => 0
                    };

                    let amount = usize::from_str(amount_str).unwrap();

                    if input_value < amount {
                        current_label = rule.1.as_ref().clone().unwrap();
                        break;
                    }
                }

                if let [label_str, amount_str] = &rule.0.split(">").map(String::from).collect::<Vec<String>>()[..] {
                    let input_value = match input_condition.iter().find(|&x| x.0.eq(label_str)) {
                        Some(x) => x.1,
                        None => 0
                    };

                    let amount = usize::from_str(amount_str).unwrap();

                    if input_value > amount {
                        current_label = rule.1.as_ref().clone().unwrap();
                        break;
                    }

                }

            }

        }

        return current_label == "A";
    }

}

#[derive(Debug)]
struct MachineError;

impl FromStr for Machine {
    type Err = MachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let mut split = s.split("\n\n");

        let workflows: Vec<Workflow> = split.nth(0).unwrap().lines().filter(|s| !s.is_empty()).map(|s| Workflow::new(s)).collect();
        let input_conditions: Vec<Vec<(String, usize)>> = split.nth(0).unwrap().lines().map(|s| 
            s.trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|n| 
                {
                   let mut split = n.split('=');
                   return (split.nth(0).unwrap().to_string(), usize::from_str(split.nth(0).unwrap()).unwrap());
                }
            ).collect::<Vec<(String, usize)>>()
        ).collect();

        return Ok(Machine { workflows, input_conditions });
    }
}

struct Workflow{
    label: String,
    rules: Vec<(String, Option<String>)>
 }

impl Workflow {
    pub fn new(str: &str) -> Workflow {
        return Workflow::from_str(str).expect("");
    }
    
    pub fn get_arrangements(&self) -> usize {

        
        return 0;
    }
}

#[derive(Debug)]
struct WorkflowError;

impl FromStr for Workflow {
    type Err = WorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err(WorkflowError);
        }

        let mut split = s.split('{');

        let label = split.nth(0).unwrap().to_string();
        let rules = split.nth(0).unwrap().trim_end_matches('}').split(',').map(|s| {

            let mut split = s.split(':');

            let condition = split.nth(0).unwrap().to_string();
            let destination = match split.nth(0) {
                Some(s) => Some(s.to_string()),
                None => None
            };

            return (condition, destination);

        }

        ).collect();

        return Ok(Workflow { label, rules });
    }
}


#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 19114);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 480738);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 167409079868000);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 0);
}