use md5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("abcdef", "00000"), 609043);
    }
}

fn solve(input: &str, start_with: &str) -> i32 {
    let mut counter = 1;

    let mut digest = md5::compute(format!("{}{}", input, counter));
    let mut hash_as_string = format!("{:x}", digest);

    while !hash_as_string.starts_with(start_with) {
        counter += 1;

        digest = md5::compute(format!("{}{}", input, counter));
        hash_as_string = format!("{:x}", digest);
    }

    counter
}

fn main() {
    println!("P1: {}", solve("iwrupvqb", "00000"));
    println!("P2: {}", solve("iwrupvqb", "000000"));
}
