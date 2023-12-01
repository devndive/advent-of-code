use std::fs;

fn replacer(input: String) -> String {
    if input.len() == 0 {
        return input;
    }

    let mut mutatedInput = input.clone();

    if mutatedInput.starts_with("one") {
        mutatedInput = mutatedInput.replace("one", "1");
    } else if mutatedInput.starts_with("two") {
        mutatedInput = mutatedInput.replace("two", "2");
    } else if mutatedInput.starts_with("three") {
        mutatedInput = mutatedInput.replace("three", "3");
    } else if mutatedInput.starts_with("four") {
        mutatedInput = mutatedInput.replace("four", "4");
    } else if mutatedInput.starts_with("five") {
        mutatedInput = mutatedInput.replace("five", "5");
    } else if mutatedInput.starts_with("six") {
        mutatedInput = mutatedInput.replace("six", "6");
    } else if mutatedInput.starts_with("seven") {
        mutatedInput = mutatedInput.replace("seven", "7");
    } else if mutatedInput.starts_with("eight") {
        mutatedInput = mutatedInput.replace("eight", "8");
    } else if mutatedInput.starts_with("nine") {
        mutatedInput = mutatedInput.replace("nine", "9");
    }

    let (start, rest) = mutatedInput.split_at(1);
    return format!("{}{}", start, replacer(String::from(rest)));
}

fn replacer_right(input: String) -> String {
    if input.len() == 0 {
        return input;
    }

    let mut mutatedInput = input.clone();

    if mutatedInput.starts_with("eno") {
        mutatedInput = mutatedInput.replace("eno", "1");
    } else if mutatedInput.starts_with("owt") {
        mutatedInput = mutatedInput.replace("owt", "2");
    } else if mutatedInput.starts_with("eerht") {
        mutatedInput = mutatedInput.replace("eerht", "3");
    } else if mutatedInput.starts_with("ruof") {
        mutatedInput = mutatedInput.replace("ruof", "4");
    } else if mutatedInput.starts_with("evif") {
        mutatedInput = mutatedInput.replace("evif", "5");
    } else if mutatedInput.starts_with("xis") {
        mutatedInput = mutatedInput.replace("xis", "6");
    } else if mutatedInput.starts_with("neves") {
        mutatedInput = mutatedInput.replace("neves", "7");
    } else if mutatedInput.starts_with("thgie") {
        mutatedInput = mutatedInput.replace("thgie", "8");
    } else if mutatedInput.starts_with("enin") {
        mutatedInput = mutatedInput.replace("enin", "9");
    }

    let (start, rest) = mutatedInput.split_at(1);
    return format!("{}{}", start, replacer_right(String::from(rest)));
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let numbers = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let from_left = replacer(String::from(line));
            let from_right = replacer_right(String::from_iter(line.chars().rev().into_iter()));
            
            let numbers_from_left = from_left.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
            let numbers_from_right = from_right.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();

            let first = numbers_from_left.first().unwrap();
            let last = numbers_from_right.first().unwrap();

            let combined = format!("{}{}", first, last);
            // println!("{} -> {} = {}", line, replaced_line, combined);
            println!("{}", combined);

            combined.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();

    let sum = numbers.iter().fold(0, |acc, val| acc + val);
    println!("{sum}");
}
