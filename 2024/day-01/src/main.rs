use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines: Vec<(u32, u32)> = contents
        .split("\n")
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let res: Vec<&str> = line.split_whitespace().collect();
            let l = res[0].parse::<u32>().expect("Could not parse left");
            let r = res[1].parse::<u32>().expect("Could not parse left");

            (l, r)
        })
        .collect();

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in &lines {
        let (l, r) = line;

        left.push(*l);
        right.push(*r);
    }

    left.sort();
    right.sort();

    let mut total = 0;
    for idx in 0..left.len() {
        let res = std::cmp::max(left[idx], right[idx]) - std::cmp::min(left[idx], right[idx]);
        println!("{} - {} = {}", left[idx], right[idx], res);

        total += res;
    }

    println!("Part 1: {total}");

    let mut map: HashMap<u32, u32> = HashMap::new();
    for n in right {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let sum = left
        .iter()
        .fold(0, |acc, num| acc + num * map.get(num).copied().unwrap_or(0));

    println!("Part 2: {sum}");
}
