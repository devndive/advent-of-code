use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let test_case = "seed-to-soil map:\n50 98 2\n52 50 48";
        let group = Group::from(test_case);

        assert_eq!(group.name, "seed-to-soil");
        assert_eq!(group.ranges.len(), 2);

        assert_eq!(group.ranges[0].destination_range_start, 50);
        assert_eq!(group.ranges[0].source_range_start, 98);
        assert_eq!(group.ranges[0].range_length, 2);

        assert_eq!(group.ranges[1].destination_range_start, 52);
        assert_eq!(group.ranges[1].source_range_start, 50);
        assert_eq!(group.ranges[1].range_length, 48);
    }

    #[test]
    fn test_two() {
        let test_case = "seed-to-soil map:\n50 98 2\n52 50 48";
        let group = Group::from(test_case);

        assert_eq!(group.map(79), 81);
        assert_eq!(group.map(14), 14);
        assert_eq!(group.map(55), 57);
        assert_eq!(group.map(13), 13);
    }
}

struct GroupRange {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl GroupRange {
    fn from(input: &str) -> Self {
        let ranges = input
            .split(" ")
            .filter(|n| n.trim().len() > 0)
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        assert_eq!(ranges.len(), 3);

        Self {
            destination_range_start: ranges[0],
            source_range_start: ranges[1],
            range_length: ranges[2],
        }
    }

    fn is_in_range(&self, input: i64) -> bool {
        input >= self.source_range_start && input <= self.source_range_start + self.range_length - 1
    }

    fn map(&self, input: i64) -> i64 {
        input - self.source_range_start + self.destination_range_start
    }
}

struct Group {
    name: String,
    ranges: Vec<GroupRange>,
}

impl Group {
    fn from(input: &str) -> Self {
        let (name, ranges) = input.split_once("\n").unwrap();
        let name = name.replace(" map:", "");
        let name = name.trim();

        let ranges = ranges
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(|l| GroupRange::from(l))
            .collect::<Vec<GroupRange>>();

        Self {
            name: String::from(name),
            ranges,
        }
    }

    fn map(&self, input: i64) -> i64 {
        for r in self.ranges.iter() {
            if r.is_in_range(input) {
                return r.map(input);
            }
        }

        input
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(args[1].clone()).unwrap();

    let (seeds, groups) = input.split_once("\n\n").unwrap();

    let seed_numbers = seeds
        .replace("seeds: ", "")
        .split(" ")
        .filter(|n| n.trim().len() > 0)
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let groups = groups
        .split("\n\n")
        .filter(|l| l.len() > 0)
        .map(|g| Group::from(g))
        .collect::<Vec<Group>>();

    let mut locations = vec![];
    for seed in seed_numbers.iter() {
        let mut s = seed.clone();
        for g in groups.iter() {
            s = g.map(s);
        }

        locations.push(s);
    }

    let mut lowest_location = locations[0];
    for loc in locations {
        lowest_location = lowest_location.min(loc);
    }

    println!("P1: {}", lowest_location);

    /*
        Solving part 2

        - Using brute force
        - Split the input to 10 files, one file per seed and length
        - Ran the program in release mode 10 times to use multiple cores
          without having to deal with async code
    */
    let mut real_seeds = vec![];
    for i in 0..seed_numbers.len() / 2 {
        let idx = i * 2;
        for n in seed_numbers[idx]..seed_numbers[idx] + seed_numbers[idx + 1] {
            real_seeds.push(n);
        }
    }

    println!("Seeds are created. Count: {}", real_seeds.len());

    let mut locations = vec![];
    for seed in real_seeds.iter() {
        let mut s = seed.clone();
        for g in groups.iter() {
            s = g.map(s);
        }

        locations.push(s);
    }

    let mut lowest_location = locations[0]; // clone.get(real_seeds.first().unwrap()).unwrap().clone();
    for loc in locations {
        lowest_location = lowest_location.min(loc.clone());
    }

    println!("P2: {}", lowest_location);
}
