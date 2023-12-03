#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("()()"), 0);
        assert_eq!(solve("(())"), 0);
    }
    
    #[test]
    fn test_two() {
        assert_eq!(solve("((("), 3);
        assert_eq!(solve("(()(()("), 3);
    }

    #[test]
    fn test_three() {
        assert_eq!(solve("))((((("), 3);
    }

    #[test]
    fn test_four() {
        assert_eq!(solve("())"), -1);
        assert_eq!(solve("))("), -1);
    }

    #[test]
    fn test_five() {
        assert_eq!(solve(")))"), -3);
        assert_eq!(solve(")())())"), -3);
    }
}

fn solve(input: &str) -> i32 {
    input.chars().map(|c| {
        if c == '(' {
            return 1
        } else{
            return -1
        }
    })
    .sum()
}

fn solve2(input: &str) -> i32 {
    let mut current_floor = 0;

    for (cur_pos, c) in input.chars().enumerate() {
        if c == '(' { current_floor += 1 } else { current_floor -= 1 }

        if current_floor == -1 {
            return cur_pos as i32 + 1;
        }
    }

    return -1;
}

fn main() {
    let input = std::fs::read_to_string("./src/input").unwrap();
    println!("{}", input.len());

    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}
