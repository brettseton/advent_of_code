use std::{fs, str::FromStr, usize};

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
    let part_range: Vec<PartRange> = vec![
        PartRange{
            start_label: "in".to_string(),
            start_x: 1, finish_x: 4000,
            start_m: 1, finish_m: 4000,
            start_a: 1, finish_a: 4000,
            start_s: 1, finish_s: 4000 }];
    return machine.get_accepted_sum_range(&part_range);
}

#[derive(Clone)]
struct PartRange {
    start_x: usize,
    finish_x: usize,
    start_m: usize,
    finish_m: usize,
    start_a: usize,
    finish_a: usize,
    start_s: usize,
    finish_s: usize,
    start_label: String
}

impl PartRange {
    pub fn get_range(&self, s: &str) -> (usize, usize) {
        return match s {
            "x" => (self.start_x, self.finish_x),
            "m" => (self.start_m, self.finish_m),
            "a" => (self.start_a, self.finish_a),
            "s" => (self.start_s, self.finish_s),
            _ => panic!("no char")
        }
    }

    pub fn set_start(&mut self, s: &str, v: usize) {
        return match s {
            "x" => self.start_x = v,
            "m" => self.start_m = v,
            "a" => self.start_a = v,
            "s" => self.start_s = v,
            _ => panic!("no char")
        }
    }

    pub fn set_finish(&mut self, s: &str, v: usize) {
        return match s {
            "x" => self.finish_x = v,
            "m" => self.finish_m = v,
            "a" => self.finish_a = v,
            "s" => self.finish_s = v,
            _ => panic!("no char")
        }
    }
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



    pub fn get_accepted_sum_range(&self, part_ranges: &Vec<PartRange>) -> usize {

        let mut queue: Vec<PartRange> = part_ranges.iter().map(|v| v.clone()).collect();
        let mut result: Vec<PartRange> = vec![];

        while let Some(part_range) = queue.pop() {

            let workflow = self.workflows.iter().find(|&x| x.label == part_range.start_label);

            if workflow.is_none() {
                result.push(part_range);
                continue;
            }

            for rule in workflow.unwrap().rules.iter() {

                if rule.1.is_none(){
                    let part = PartRange{
                        start_label: rule.0.to_string(),
                        start_x: part_range.start_x, finish_x: part_range.finish_x,
                        start_m: part_range.start_m, finish_m: part_range.finish_m,
                        start_a: part_range.start_a, finish_a: part_range.finish_a,
                        start_s: part_range.start_s, finish_s: part_range.finish_s };
                    queue.push(part);
                    break;
                }
                
                if let [rule_variable_str, amount_str] = &rule.0.split("<").map(String::from).collect::<Vec<String>>()[..] {

                    let rule_amount = usize::from_str(amount_str).unwrap();

                    let (start_range, end_range) = part_range.get_range(rule_variable_str);
                    // Split range
                    if rule_amount > start_range && rule_amount < end_range  {
                        let mut part1 = part_range.clone();
                        part1.set_finish(rule_variable_str, rule_amount - 1);
                        part1.start_label = rule.1.as_ref().unwrap().to_string();
                        queue.push(part1.clone());

                        let mut part2 = part_range.clone();
                        part2.set_start(rule_variable_str, rule_amount);
                        part2.start_label = part_range.start_label.to_string();
                        queue.push(part2);


                        break;

                    } else if end_range < rule_amount {
                        let mut part = part_range.clone();
                        part.start_label = rule.1.as_ref().unwrap().to_string();
                        queue.push(part);
                        break;
                    }
                }

                if let [rule_variable_str, amount_str] = &rule.0.split(">").map(String::from).collect::<Vec<String>>()[..] {

                    
                    let rule_amount = usize::from_str(amount_str).unwrap();

                    let (start_range, end_range) = part_range.get_range(rule_variable_str);
                    // Split range
                    if rule_amount > start_range && rule_amount < end_range  {
                        let mut part1 = part_range.clone();
                        part1.set_finish(rule_variable_str, rule_amount);
                        part1.start_label = part_range.start_label.to_string();
                        queue.push(part1.clone());

                        let mut part2 = part_range.clone();
                        part2.set_start(rule_variable_str, rule_amount + 1);
                        part2.start_label = rule.1.as_ref().unwrap().to_string();
                        queue.push(part2);


                        break;

                    } else if start_range > rule_amount {
                        let mut part = part_range.clone();
                        part.start_label = rule.1.as_ref().unwrap().to_string();
                        queue.push(part);
                        break;
                    }
                }
            }
        }
        
        let sum = result.iter()
        .map(|x| {
                if x.start_label == "A" {
                    return (x.finish_x - x.start_x + 1) * (x.finish_m - x.start_m + 1) * (x.finish_a - x.start_a + 1) * (x.finish_s - x.start_s + 1);
                }
                return 0;
            }).sum::<usize>();


        return sum;
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
    assert_eq!(ans, 131550418841958);
}