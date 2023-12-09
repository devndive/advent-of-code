#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(next_seq("0 3 6 9 12 15").0, 18);
        assert_eq!(next_seq("1 3 6 10 15 21").0, 28);
        assert_eq!(next_seq("10 13 16 21 30 45").0, 68);
    }

    #[test]
    fn test_two() {
        assert_eq!(next_seq("0 3 6 9 12 15").1, -3);
        assert_eq!(next_seq("1 3 6 10 15 21").1, 0);
        assert_eq!(next_seq("10 13 16 21 30 45").1, 5);
    }
}

fn next_seq(input: &str) -> (i32, i32) {
    let n = input
        .split(" ")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut sequences: Vec<Vec<i32>> = [].to_vec();
    sequences.push(n.clone());

    loop {
        let to_process = sequences.last().unwrap();

        let mut next_seq = vec![];
        for idx in 0..to_process.len() - 1 {
            let n1 = to_process.iter().nth(idx).unwrap();
            let n2 = to_process.iter().nth(idx + 1).unwrap();

            next_seq.push(n2 - n1);
        }

        sequences.push(next_seq.clone());

        if next_seq.iter().all(|n| *n == 0) {
            break;
        }
    }

    let next_step = sequences
        .iter()
        .fold(0, |acc, val| {
            acc + val.last().unwrap()
        });

    let prev_step = sequences
        .iter()
        .rev()
        .fold(0, |acc, val| {
            val.first().unwrap() - acc
        });

    (next_step, prev_step)
}

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();

    let sum = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| next_seq(l).0)
        .sum::<i32>();

    println!("P1: {}", sum);

    let sum = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| next_seq(l).1)
        .sum::<i32>();

    println!("P2: {}", sum);
}
