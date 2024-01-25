use std::{fs, str::FromStr, collections::{HashMap, HashSet}};

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 : {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let wire_diagram = WireDiagram::new(&input);
    return wire_diagram.get_sum();
}

struct WireDiagram {
    graph: HashMap<String, HashSet<String>>,
}

impl WireDiagram {
    pub fn new(str: &str) -> WireDiagram {
        return WireDiagram::from_str(str).expect("");
    }

    fn get_sum(&self) -> usize {
        let mut nodes: HashSet<String> = self.graph.keys().cloned().collect();
        let count = |v: &String, nodes: &HashSet<String>| -> usize { self.graph[v].difference(nodes).count() };

        while nodes.iter().map(|v| count(v, &nodes)).sum::<usize>() != 3 {
            let max_v = nodes.iter().max_by_key(|v| count(v, &nodes)).cloned().unwrap();
            nodes.remove(&max_v);
        }

        return nodes.len() * (self.graph.keys().len() - &nodes.len());
    }
}

#[derive(Debug)]
struct WireDiagramError;

impl FromStr for WireDiagram {
    type Err = WireDiagramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        s.lines().for_each(|s| {
            let [label_str, children_str] =
                &s.split(':').map(String::from).collect::<Vec<String>>()[..]
            else {
                panic!()
            };
            let links = children_str
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();

            for link in links.iter() {
                let entry = graph.entry(link.to_string()).or_insert(HashSet::new());
                entry.insert(label_str.to_string());
                let new_entry = graph.entry(label_str.to_string()).or_insert(HashSet::new());
                new_entry.insert(link.to_string());
            }
        });
        return Ok(WireDiagram { graph });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 54);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 562912);
}
