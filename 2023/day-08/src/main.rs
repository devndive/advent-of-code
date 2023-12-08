use std::collections::HashMap;
use regex::Regex;

fn p1(hash_maps: HashMap<&str, (&str, &str)>, instruction: &str) {
    let mut next_node = "AAA";
    let mut steps = 0;

    loop {
        let direction_pos = steps % instruction.len();
        let direction = instruction.chars().nth(direction_pos).unwrap();

        next_node = match direction {
            'L' => hash_maps.get(next_node).unwrap().0,
            'R' => hash_maps.get(next_node).unwrap().1,
            _ => panic!("Whoopsie - {direction}"),
        };

        steps += 1;

        if next_node == "ZZZ" {
            break;
        }
    }

    println!("Steps: {}", steps);
}

fn p2(hash_maps: HashMap<&str, (&str, &str)>, instruction: &str, start_node: &str) -> i64 {
    let mut next_node = start_node;
    let mut steps = 0;

    loop {
        let direction_pos = steps % instruction.len();
        let direction = instruction.chars().nth(direction_pos).unwrap();

        next_node = match direction {
            'L' => hash_maps.get(next_node).unwrap().0,
            'R' => hash_maps.get(next_node).unwrap().1,
            _ => panic!("Whoopsie - {direction}"),
        };

        steps += 1;

        if next_node.ends_with("Z") {
            break;
        }
    }

    steps as i64
}

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();
    let (instruction, maps) = input.split_once("\n\n").unwrap();

    let maps = maps
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| {
            let re: Regex = Regex::new(r"[0-9A-Z]{3}").unwrap();
            let res = re.find_iter(l).map(|mat| mat.as_str()).collect::<Vec<&str>>();

            (res[0], res[1], res[2])
        })
        .collect::<Vec<(&str, &str, &str)>>();

    let mut hash_maps = HashMap::new();
    for m in maps {
        hash_maps.insert(m.0, (m.1, m.2));
    }

    // p1(hash_maps.clone(), instruction);

    let mut steps = hash_maps
        .iter()
        .filter(|h| h.0.ends_with("A"))
        .map(|h| p2(hash_maps.clone(), instruction, h.0))
        .collect::<Vec<i64>>();

    steps.sort();

    for s in steps.iter() {
        println!("{}", s);
        // Too stupid to calculate kgV
        // used
        // https://www.matheretter.de/rechner/kgv
    }
}
