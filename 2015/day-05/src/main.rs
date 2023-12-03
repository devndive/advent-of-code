
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);

        assert_eq!(is_nice("uxcplgxnkwbdwhrp"), false);
        assert_eq!(is_nice("mllyabngqmzfcubp"), false);
    }
}


fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn is_nice(input: &str) -> bool {

    let char_vec = input.chars().collect::<Vec<char>>();

    let vowels = input.chars().filter(|c| is_vowel(*c)).collect::<Vec<char>>();
    let at_least_three_vowels = vowels.len() >= 3;

    let mut at_least_one_double_char = false;
    for (pos, c) in input.chars().enumerate() {
        if pos + 1 == input.len() { break; }

        if c == char_vec[pos+1] {
            at_least_one_double_char = true;
            break;
        }
    }

    let should_not_be_there = ["ab", "cd", "pq", "xy"];
    let contains_any = should_not_be_there.iter().all(|s| !input.contains(s));

    at_least_three_vowels && at_least_one_double_char && contains_any
}

fn main() {
    let input = std::fs::read_to_string("./src/input").unwrap();

    let numbers = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| {
            println!("{} - {}", l, is_nice(l));
            is_nice(l)
        })
        .filter(|r| *r)
        .count();

    println!("P1: {}", numbers);
}
