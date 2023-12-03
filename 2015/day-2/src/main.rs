#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "2x3x4";
        assert_eq!(solve(input), 58);
    }

    #[test]
    fn test_two() {
        let input = "1x1x10";
        assert_eq!(solve(input), 43);
    }
}

fn solve(input: &str) -> i32 {
    let parts = input.split("x")
        .map(|p| {
            p.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();
    let l = parts[0];
    let w = parts[1];
    let h = parts[2];

    let sides = [l*w, w*h, h*l];
    let samllest_side = sides.iter().min().unwrap();

    sides.iter().sum::<i32>() * 2 + samllest_side
}

fn solve2(input: &str) -> i32 {
    let parts = input.split("x")
        .map(|p| {
            p.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();
    let l = parts[0];
    let w = parts[1];
    let h = parts[2];

    let mut sides = [l, w, h];
    sides.sort();

    sides[0] * 2 + sides[1] * 2 + l * w * h
}

fn main() {
    let input = std::fs::read_to_string("./src/input").unwrap();

    let wrapping_paper = input.split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| solve(l))
        .sum::<i32>();

    println!("{}", wrapping_paper);

    let ribbon = input.split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| solve2(l))
        .sum::<i32>();

    println!("{}", ribbon);
}
