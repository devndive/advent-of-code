#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_save() {
        let actual = is_save(&vec![7, 6, 4, 2, 1]);
        assert_eq!(actual, true);
        let actual = is_save(&vec![1, 2, 7, 8, 9]);
        assert_eq!(actual, false);
        let actual = is_save(&vec![9, 7, 6, 2, 1]);
        assert_eq!(actual, false);
        let actual = is_save(&vec![1, 3, 2, 4, 5]);
        assert_eq!(actual, false);
        let actual = is_save(&vec![8, 6, 4, 4, 1]);
        assert_eq!(actual, false);
        let actual = is_save(&vec![1, 3, 6, 7, 9]);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_is_save_with_retry() {
        let actual = is_save_with_retry(&vec![7, 6, 4, 2, 1]);
        assert_eq!(actual, true);
        let actual = is_save_with_retry(&vec![1, 2, 7, 8, 9]);
        assert_eq!(actual, false);
        let actual = is_save_with_retry(&vec![9, 7, 6, 2, 1]);
        assert_eq!(actual, false);
        let actual = is_save_with_retry(&vec![1, 3, 2, 4, 5]);
        assert_eq!(actual, true);
        let actual = is_save_with_retry(&vec![8, 6, 4, 4, 1]);
        assert_eq!(actual, true);
        let actual = is_save_with_retry(&vec![1, 3, 6, 7, 9]);
        assert_eq!(actual, true);
        let actual = is_save_with_retry(&vec![99, 96, 93, 90, 88, 86, 81, 83]);
        assert_eq!(actual, true);
        let actual = is_save_with_retry(&vec![48, 46, 47, 49, 51, 54, 56]);
        assert_eq!(actual, true);
    }
}

use std::env;
use std::fs;

fn is_save(report: &Vec<u32>) -> bool {
    if report[0] == report[1] {
        return false;
    }

    // From high to low
    if report[0] > report[1] {
        for idx in 0..report.len() - 1 {
            if report[idx] <= report[idx + 1]
                || report[idx].abs_diff(report[idx + 1]) < 1
                || report[idx].abs_diff(report[idx + 1]) > 3
            {
                return false;
            }
        }
    // From low to high
    } else {
        for idx in 0..report.len() - 1 {
            if report[idx] >= report[idx + 1]
                || report[idx].abs_diff(report[idx + 1]) < 1
                || report[idx].abs_diff(report[idx + 1]) > 3
            {
                return false;
            }
        }
    }

    true
}

fn is_save_with_retry(levels: &Vec<u32>) -> bool {
    let result = is_save(levels);

    if !result {
        for idx in 0..levels.len() {
            let mut retry = levels.clone();
            retry.remove(idx);

            let result_after_retry = is_save(&retry);

            if result_after_retry {
                return result_after_retry;
            }
        }
    }

    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let reports: Vec<Vec<u32>> = contents
        .split("\n")
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let res: Vec<u32> = line
                .split_whitespace()
                .map(|item| item.parse::<u32>().expect("Could not parse number"))
                .collect();

            res
        })
        .collect();

    let count = reports.iter().filter(|report| is_save(&report)).count();
    println!("Save reports: {count}");

    let count = reports
        .iter()
        .filter(|report| is_save_with_retry(&report))
        .count();
    println!("Save reports with retry: {count}");
}
