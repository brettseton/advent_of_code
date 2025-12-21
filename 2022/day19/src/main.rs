use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl Resource {
    const COUNT: usize = 4;
    const ALL: [Resource; 4] = [
        Resource::Ore,
        Resource::Clay,
        Resource::Obsidian,
        Resource::Geode,
    ];
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: i32,
    costs: [[i32; Resource::COUNT]; Resource::COUNT],
    max_robots: [i32; Resource::COUNT],
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<i32> = s
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() < 7 {
            return Err(format!("Invalid blueprint string: {}", s));
        }

        let mut costs = [[0; Resource::COUNT]; Resource::COUNT];
        costs[Resource::Ore as usize][Resource::Ore as usize] = nums[1];
        costs[Resource::Clay as usize][Resource::Ore as usize] = nums[2];
        costs[Resource::Obsidian as usize][Resource::Ore as usize] = nums[3];
        costs[Resource::Obsidian as usize][Resource::Clay as usize] = nums[4];
        costs[Resource::Geode as usize][Resource::Ore as usize] = nums[5];
        costs[Resource::Geode as usize][Resource::Obsidian as usize] = nums[6];

        let mut max_robots = [i32::MAX; Resource::COUNT];
        for res_idx in 0..3 {
            max_robots[res_idx] = costs.iter().map(|c| c[res_idx]).max().unwrap_or(0);
        }

        Ok(Self {
            id: nums[0],
            costs,
            max_robots,
        })
    }
}

impl Blueprint {
    fn solve(&self, time_limit: i32) -> i32 {
        let mut max_geodes = 0;
        self.dfs(State::initial(time_limit), &mut max_geodes);
        max_geodes
    }

    fn dfs(&self, state: State, max_geodes: &mut i32) {
        let current_max_at_end = state.potential_geodes_at_end();
        *max_geodes = (*max_geodes).max(current_max_at_end);

        if state.time <= 1 {
            return;
        }

        let max_possible = current_max_at_end + (state.time * (state.time - 1)) / 2;
        if max_possible <= *max_geodes {
            return;
        }

        for &rtype in Resource::ALL.iter().rev() {
            let r_idx = rtype as usize;

            if state.robots[r_idx] >= self.max_robots[r_idx] {
                continue;
            }

            if let Some(wait_time) = state.wait_time_to_build(rtype, &self.costs[r_idx]) {
                if state.time > wait_time + 1 {
                    let next_state = state.build(rtype, wait_time, &self.costs[r_idx]);
                    self.dfs(next_state, max_geodes);

                    if rtype == Resource::Geode && wait_time == 0 {
                        return;
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct State {
    time: i32,
    resources: [i32; Resource::COUNT],
    robots: [i32; Resource::COUNT],
}

impl State {
    fn initial(time: i32) -> Self {
        Self {
            time,
            resources: [0; Resource::COUNT],
            robots: [1, 0, 0, 0],
        }
    }

    fn potential_geodes_at_end(&self) -> i32 {
        let geode_idx = Resource::Geode as usize;
        self.resources[geode_idx] + self.robots[geode_idx] * self.time
    }

    fn wait_time_to_build(&self, _rtype: Resource, costs: &[i32; Resource::COUNT]) -> Option<i32> {
        let mut max_wait = 0;
        for (res_idx, &cost) in costs.iter().enumerate().take(3) {
            if cost > 0 {
                if self.robots[res_idx] == 0 {
                    return None;
                }
                let needed = (cost - self.resources[res_idx]).max(0);
                let wait = (needed + self.robots[res_idx] - 1) / self.robots[res_idx];
                max_wait = max_wait.max(wait);
            }
        }
        Some(max_wait)
    }

    fn build(&self, rtype: Resource, wait_time: i32, costs: &[i32; Resource::COUNT]) -> Self {
        let mut next = *self;
        let elapsed = wait_time + 1;
        next.time -= elapsed;
        for i in 0..Resource::COUNT {
            next.resources[i] += self.robots[i] * elapsed;
        }
        for (res_idx, &cost) in costs.iter().enumerate().take(3) {
            next.resources[res_idx] -= cost;
        }
        next.robots[rtype as usize] += 1;
        next
    }
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    input
        .split("Blueprint ")
        .filter(|s| !s.trim().is_empty())
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn part1(input: &str) -> i32 {
    parse_blueprints(input)
        .iter()
        .map(|bp| bp.id * bp.solve(24))
        .sum()
}

fn part2(input: &str) -> i32 {
    parse_blueprints(input)
        .iter()
        .take(3)
        .map(|bp| bp.solve(32))
        .product()
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));
    println!("Part 2 test 1: {}", part2(TEST_INPUT_1));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 33);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 1294);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 3472);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 13640);
    }
}
