use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Gate {
    And(String, String, String), // (input1, input2, output)
    Or(String, String, String),  // (input1, input2, output)
    Xor(String, String, String), // (input1, input2, output)
}

impl Gate {
    fn inputs(&self) -> (&str, &str) {
        match self {
            Gate::And(in1, in2, _) | Gate::Or(in1, in2, _) | Gate::Xor(in1, in2, _) => (in1, in2),
        }
    }

    fn output(&self) -> &str {
        match self {
            Gate::And(_, _, out) | Gate::Or(_, _, out) | Gate::Xor(_, _, out) => out,
        }
    }
}

type Circuit = HashMap<String, bool>;

fn parse_input(input: &str) -> (Circuit, Vec<Gate>) {
    let mut initial_values = HashMap::new();
    let mut gates = Vec::new();
    let mut parsing_gates = false;

    for line in input.lines() {
        if line.is_empty() {
            parsing_gates = true;
            continue;
        }

        if !parsing_gates {
            let (wire, value) = line.split_once(": ").expect("Invalid initial value format");
            initial_values.insert(wire.to_string(), value.parse::<u8>().unwrap() == 1);
        } else {
            let (gate_def, output) = line.split_once(" -> ").expect("Invalid gate format");
            let parts: Vec<&str> = gate_def.split_whitespace().collect();

            if parts.len() == 3 {
                let gate = match parts[1] {
                    "AND" => Gate::And(
                        parts[0].to_string(),
                        parts[2].to_string(),
                        output.trim().to_string(),
                    ),
                    "OR" => Gate::Or(
                        parts[0].to_string(),
                        parts[2].to_string(),
                        output.trim().to_string(),
                    ),
                    "XOR" => Gate::Xor(
                        parts[0].to_string(),
                        parts[2].to_string(),
                        output.trim().to_string(),
                    ),
                    op => panic!("Unknown gate operator: {op}"),
                };
                gates.push(gate);
            }
        }
    }

    (initial_values, gates)
}

fn simulate_circuit(initial_values: &Circuit, gates: &[Gate]) -> Circuit {
    let mut values = initial_values.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for gate in gates {
            let (in1, in2) = gate.inputs();
            if let (Some(&v1), Some(&v2)) = (values.get(in1), values.get(in2)) {
                let result = match gate {
                    Gate::And(_, _, _) => v1 && v2,
                    Gate::Or(_, _, _) => v1 || v2,
                    Gate::Xor(_, _, _) => v1 ^ v2,
                };

                let out = gate.output();
                if values.get(out).copied() != Some(result) {
                    values.insert(out.to_string(), result);
                    changed = true;
                }
            }
        }
    }

    values
}

fn calculate_result(values: &Circuit) -> i64 {
    let mut result = 0;
    let mut bit = 0;

    while let Some(&value) = values.get(&format!("z{:02}", bit)) {
        if value {
            result |= 1 << bit;
        }
        bit += 1;
    }

    result
}

fn find_highest_z_wire(gates: &[Gate]) -> String {
    gates
        .iter()
        .filter_map(|gate| {
            let out = gate.output();
            if out.starts_with('z') {
                // Parse the number after 'z'
                out.strip_prefix('z')
                    .and_then(|num_str| num_str.parse::<u32>().ok())
                    .map(|num| (num, out.to_string()))
            } else {
                None
            }
        })
        .max_by_key(|(num, _)| *num)
        .map(|(_, wire)| wire)
        .unwrap_or_else(|| "z00".to_string())
}

fn find_faulty_gates(gates: &[Gate]) -> Vec<String> {
    let mut faulty_gates = Vec::new();
    let last_z_wire = find_highest_z_wire(gates);

    for gate in gates {
        let (in1, in2) = gate.inputs();
        let out = gate.output();

        // Case 1: Z-wire outputs should use XOR (except final bit)
        if out.starts_with('z') && out != last_z_wire && !matches!(gate, Gate::Xor(_, _, _)) {
            faulty_gates.push(out.to_string());
        }

        // Case 2: Non-z outputs with non-x/y inputs should use AND or OR
        let has_xy_input = in1.starts_with('x')
            || in1.starts_with('y')
            || in2.starts_with('x')
            || in2.starts_with('y');
        if !out.starts_with('z') && !has_xy_input && matches!(gate, Gate::Xor(_, _, _)) {
            faulty_gates.push(out.to_string());
        }

        // Case 3: XOR gates with x/y inputs (except bit 0) should feed into another XOR
        if matches!(gate, Gate::Xor(_, _, _))
            && has_xy_input
            && !(in1.ends_with("00") && in2.ends_with("00"))
        {
            let feeds_into_xor = gates.iter().any(|other| {
                matches!(other, Gate::Xor(other_in1, other_in2, _) if other_in1 == out || other_in2 == out)
            });
            if !feeds_into_xor {
                faulty_gates.push(out.to_string());
            }
        }

        // Case 4: AND gates with x/y inputs (except bit 0) should feed into OR
        if matches!(gate, Gate::And(_, _, _))
            && has_xy_input
            && !(in1.ends_with("00") && in2.ends_with("00"))
        {
            let feeds_into_or = gates.iter().any(|other| {
                matches!(other, Gate::Or(other_in1, other_in2, _) if other_in1 == out || other_in2 == out)
            });
            if !feeds_into_or {
                faulty_gates.push(out.to_string());
            }
        }
    }

    faulty_gates.sort();
    faulty_gates.dedup();
    faulty_gates
}

fn part1(input: &str) -> i64 {
    let (initial_values, gates) = parse_input(input);
    let final_values = simulate_circuit(&initial_values, &gates);
    calculate_result(&final_values)
}

fn part2(input: &str) -> String {
    let (_initial_values, gates) = parse_input(input);
    let faulty_gates = find_faulty_gates(&gates);
    faulty_gates.join(",")
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));

    println!("Part 2 test 1: {}", part2(&input1));
    println!("Part 2 test 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 2024);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 69201640933606);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(
            part2(&test_input),
            "ffh,mjb,tgd,wpb,z02,z03,z05,z06,z07,z08,z10,z11"
        );
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        // Update with actual expected value
        assert_eq!(part2(&test_input), "dhq,hbs,jcp,kfp,pdg,z18,z22,z27");
    }
}
