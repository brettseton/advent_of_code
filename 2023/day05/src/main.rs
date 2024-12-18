use std::fs;
use std::str::FromStr;

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let almanac = Almanac::new(&input);
    let seed_destinations = almanac.get_seed_destinations();
    return seed_destinations
        .into_iter()
        .min()
        .expect("there will be a map");
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let almanac = Almanac::new(&input);
    let seed_destinations = almanac.get_seed_range_destinations();

    let min = seed_destinations.iter().map(|b| b.start).min().unwrap_or(0);
    return min;
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn new(str: &str) -> Almanac {
        return Almanac::from_str(str).expect("Ctor from string failed");
    }

    pub fn get_seed_destinations(&self) -> Vec<usize> {
        let mut destinations = Vec::new();
        let mut seed_destination: usize;
        for seed in self.seeds.iter() {
            seed_destination = *seed;
            for map in self.maps.iter() {
                seed_destination = map.get_map_destination(seed_destination);
            }
            destinations.push(seed_destination);
        }

        return destinations;
    }

    pub fn get_seed_range_destinations(&self) -> Vec<SeedRange> {
        let mut destinations = Vec::new();
        for chunks in self.seeds.chunks(2) {
            let seed_start = chunks[0];
            let seed_range = chunks[1];
            let mut seed_ranges = vec![SeedRange {
                start: seed_start,
                range: seed_range,
            }];
            for map in self.maps.iter() {
                seed_ranges = map.get_map_destination_ranges(seed_ranges);
            }
            destinations.extend(seed_ranges.into_iter());
        }
        return destinations;
    }
}

#[derive(Debug)]
struct AlmanacParseError;

impl FromStr for Almanac {
    type Err = AlmanacParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let seeds: Vec<usize>;
        let maps: Vec<Map>;
        if let [seed_str, seed_to_soil_str, soil_to_fertilizer_str, fertilizer_to_water_str, water_to_light_str, light_to_temperature_str, temperature_to_humidity_str, humidity_to_location_str] =
            &str.split("\n\n").map(String::from).collect::<Vec<String>>()[..]
        {
            seeds = seed_str
                .split(':')
                .nth(1)
                .expect("some seeds")
                .split_whitespace()
                .map(|x| x.parse::<usize>().expect("Unable to parse seeds"))
                .collect();
            let seed_to_soil_map = Map::new(seed_to_soil_str);
            let soil_to_fertilizer_map = Map::new(soil_to_fertilizer_str);
            let fertilizer_to_water_map = Map::new(fertilizer_to_water_str);
            let water_to_light_map = Map::new(water_to_light_str);
            let light_to_temperature_map = Map::new(light_to_temperature_str);
            let temperature_to_humidity_map = Map::new(temperature_to_humidity_str);
            let humidity_to_location_map = Map::new(humidity_to_location_str);

            maps = vec![
                seed_to_soil_map,
                soil_to_fertilizer_map,
                fertilizer_to_water_map,
                water_to_light_map,
                light_to_temperature_map,
                temperature_to_humidity_map,
                humidity_to_location_map,
            ];
        } else {
            return Err(AlmanacParseError);
        }

        return Ok(Almanac { seeds, maps });
    }
}

struct SeedRange {
    start: usize,
    range: usize,
}

struct Map {
    source_to_destinations: Vec<SourceToDestination>,
}

impl Map {
    pub fn new(str: &str) -> Map {
        return Map::from_str(str).expect("Ctor from string failed");
    }

    pub fn get_map_destination(&self, start: usize) -> usize {
        for source_to_destination in self.source_to_destinations.iter() {
            if start >= source_to_destination.source_start
                && start <= source_to_destination.source_start + source_to_destination.range_length
            {
                let destination = start - source_to_destination.source_start
                    + source_to_destination.destination_start;
                return destination;
            }
        }

        return start;
    }

    pub fn get_map_destination_ranges(&self, seed_ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        let mut all_destination_ranges = Vec::new();
        for seed_range in seed_ranges {
            let mut destination_ranges = Vec::new();
            for source_to_destination in self.source_to_destinations.iter() {
                let start = source_to_destination.source_start;
                let end =
                    source_to_destination.source_start + source_to_destination.range_length - 1;

                let seed_start = seed_range.start;
                let seed_end = seed_range.start + seed_range.range - 1;

                // Do the windows overlap
                if seed_end >= start && seed_start <= end {
                    // Remap everything in current seed range
                    if seed_start >= start && seed_end <= end {
                        // No Split required as full range is mapped
                        let new_start =
                            seed_start - start + source_to_destination.destination_start;
                        let new_range = seed_range.range;
                        destination_ranges.push(SeedRange {
                            start: new_start,
                            range: new_range,
                        });
                        break;
                    } else if seed_start >= start && seed_end > end {
                        // Split into two list
                        // 1. middle full overlap
                        // 2. overhang at end

                        // 1. middle full overlap
                        // Push the original list up to its max

                        let new_start =
                            seed_start - start + source_to_destination.destination_start;
                        let new_range = end - seed_start + 1;

                        destination_ranges.push(SeedRange {
                            start: new_start,
                            range: new_range,
                        });

                        // 2. overhang at end
                        // Find the destination for the remainder of the list
                        let split_start = end + 1;
                        let split_range = seed_end - end;

                        let mut remaining_destinations =
                            self.get_map_destination_ranges(vec![SeedRange {
                                start: split_start,
                                range: split_range,
                            }]);

                        destination_ranges.append(&mut remaining_destinations);
                        break;
                    } else if seed_start < start && seed_end <= end {
                        // Split into two list
                        // 1. overhang at start
                        // 2. middle full overlap

                        // 1. overhang at start
                        let split_start = seed_start;
                        let split_range = start - seed_start;

                        let mut remaining_destinations =
                            self.get_map_destination_ranges(vec![SeedRange {
                                start: split_start,
                                range: split_range,
                            }]);
                        destination_ranges.append(&mut remaining_destinations);

                        // 2. middle full overlap
                        let new_start = source_to_destination.destination_start;
                        let new_range = seed_end - start + 1;

                        destination_ranges.push(SeedRange {
                            start: new_start,
                            range: new_range,
                        });

                        break;
                    } else if seed_start < start && seed_end > end {
                        // Split into three list
                        // 1. overhang at start
                        // 2. middle full overlap
                        // 3. overhang at end

                        // 1. overhang at start
                        let split_start = seed_start;
                        let split_range = start - seed_start;

                        let mut remaining_destinations =
                            self.get_map_destination_ranges(vec![SeedRange {
                                start: split_start,
                                range: split_range,
                            }]);
                        destination_ranges.append(&mut remaining_destinations);

                        // 2. middle full overlap
                        let new_start = source_to_destination.destination_start;
                        let new_range = end - start + 1;

                        destination_ranges.push(SeedRange {
                            start: new_start,
                            range: new_range,
                        });

                        // 3. overhang at end
                        let split_start = end + 1;
                        let split_range = seed_end - end;

                        let mut remaining_destinations =
                            self.get_map_destination_ranges(vec![SeedRange {
                                start: split_start,
                                range: split_range,
                            }]);
                        destination_ranges.append(&mut remaining_destinations);

                        break;
                    }
                }
            }

            // No mappings found
            if destination_ranges.is_empty() {
                destination_ranges.push(SeedRange {
                    start: seed_range.start,
                    range: seed_range.range,
                });
            }
            all_destination_ranges.append(&mut destination_ranges);
        }
        return all_destination_ranges;
    }
}

#[derive(Debug)]
struct MapsParseError;

impl FromStr for Map {
    type Err = MapsParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut source_to_destinations: Vec<SourceToDestination> = Vec::new();
        // Skip 'seed-to-soil map:' name line
        for line in str.lines().skip(1) {
            source_to_destinations.push(SourceToDestination::new(line));
        }

        return Ok(Map {
            source_to_destinations,
        });
    }
}

struct SourceToDestination {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl SourceToDestination {
    pub fn new(str: &str) -> SourceToDestination {
        return SourceToDestination::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct SourceToDestinationParseError;

impl FromStr for SourceToDestination {
    type Err = SourceToDestinationParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let [destination_start, source_start, range_length] = str
            .split_whitespace()
            .map(|x| x.parse::<usize>().expect("Unable to parse ranges"))
            .collect::<Vec<usize>>()[..]
        {
            return Ok(SourceToDestination {
                destination_start,
                source_start,
                range_length,
            });
        }

        return Err(SourceToDestinationParseError);
    }
}

#[test]
fn part1_test1() {
    let result = part1("input/test1.txt");
    assert_eq!(result, 35);
}

#[test]
fn part1_test2() {
    let result = part1("input/test2.txt");
    assert_eq!(result, 278755257);
}

#[test]
fn part2_test1() {
    let result = part2("input/test1.txt");
    assert_eq!(result, 46);
}

#[test]
fn part2_test2() {
    let result = part2("input/test2.txt");
    assert_eq!(result, 26829166);
}
