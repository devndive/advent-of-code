use core::panic;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(calc_strength("22222"), Win::FIVE_OF_A_KIND);
        assert_eq!(calc_strength("AAAA2"), Win::FOUR_OF_A_KIND);
        assert_eq!(calc_strength("2AAAA"), Win::FOUR_OF_A_KIND);
        assert_eq!(calc_strength("AA2AA"), Win::FOUR_OF_A_KIND);

        assert_eq!(calc_strength("23332"), Win::FULL_HOUSE);
        assert_eq!(calc_strength("TT98T"), Win::THREE_OF_A_KIND);
        assert_eq!(calc_strength("23432"), Win::TWO_PAIR);
        assert_eq!(calc_strength("A23A4"), Win::ONE_PAIR);
        assert_eq!(calc_strength("23456"), Win::HIGH_CARD);
    }

    #[test]
    fn test_two() {
        assert_eq!(calc_strength_with_j("32T3K"), Win::ONE_PAIR);
        assert_eq!(calc_strength_with_j("KK677"), Win::TWO_PAIR);
        assert_eq!(calc_strength_with_j("T55J5"), Win::FOUR_OF_A_KIND);
        assert_eq!(calc_strength_with_j("KTJJT"), Win::FOUR_OF_A_KIND);
        assert_eq!(calc_strength_with_j("QQQJA"), Win::FOUR_OF_A_KIND);
        assert_eq!(calc_strength_with_j("QQQJQ"), Win::FIVE_OF_A_KIND);
        assert_eq!(calc_strength_with_j("JJJJJ"), Win::FIVE_OF_A_KIND);
        assert_eq!(calc_strength_with_j("Q2345"), Win::HIGH_CARD);
        assert_eq!(calc_strength_with_j("Q2KJJ"), Win::THREE_OF_A_KIND);

        assert_eq!(calc_strength_with_j("2JJJJ"), Win::FIVE_OF_A_KIND);
        assert_eq!(calc_strength_with_j("2AAAA"), Win::FOUR_OF_A_KIND);

        assert_eq!(calc_strength_with_j("2233J"), Win::FULL_HOUSE);
        assert_eq!(calc_strength_with_j("AJJ94"), Win::THREE_OF_A_KIND);
        assert_eq!(calc_strength_with_j("AAAJJ"), Win::FIVE_OF_A_KIND);
        assert_eq!(calc_strength_with_j("JJJAA"), Win::FIVE_OF_A_KIND);
    }

    #[test]
    fn test_three() {
        assert_eq!(calc_strength_with_j("QJJQ2"), Win::FOUR_OF_A_KIND);
        assert_eq!(calc_strength_with_j("QQQQ2"), Win::FOUR_OF_A_KIND);

        let mut res: Vec<(&str, i32, i32)> = vec![
            ("QQQQ2", 8, map_strength_to_points(calc_strength_with_j("QQQQ2"))),
            ("QJJQ2", 7, map_strength_to_points(calc_strength_with_j("QJJQ2"))),
        ];

        res.sort_by(|a, b| {
            let cmp = a.2.partial_cmp(&b.2).unwrap();

            if cmp.is_eq() {
                for idx in 0..a.0.chars().collect::<Vec<char>>().len() {
                    let first = map_card_to_value(a.0.chars().nth(idx).unwrap(), 2);
                    let secon = map_card_to_value(b.0.chars().nth(idx).unwrap(), 2);

                    if first.cmp(&secon).is_ne() {
                        return first.cmp(&secon);
                    }
                }
            }

            cmp
        });

        assert_eq!(res[0].0, "QJJQ2");
        assert_eq!(res[1].0, "QQQQ2");
    }

    #[test]
    fn test_four() {
        assert_eq!(calc_strength_with_j("KKK23"), Win::THREE_OF_A_KIND);
        assert_eq!(calc_strength_with_j("AJJ94"), Win::THREE_OF_A_KIND);
        assert_eq!(calc_strength_with_j("A2223"), Win::THREE_OF_A_KIND);

        let mut res: Vec<(&str, i32, i32)> = vec![
            ("A2223", 7, map_strength_to_points(calc_strength_with_j("A2223"))),
            ("KKK23", 8, map_strength_to_points(calc_strength_with_j("KKK23"))),
            ("AJJ94", 7, map_strength_to_points(calc_strength_with_j("AJJ94"))),
        ];

        res.sort_by(|a, b| {
            let cmp = a.2.partial_cmp(&b.2).unwrap();

            if cmp.is_eq() {
                for idx in 0..a.0.chars().collect::<Vec<char>>().len() {
                    let first = map_card_to_value(a.0.chars().nth(idx).unwrap(), 2);
                    let secon = map_card_to_value(b.0.chars().nth(idx).unwrap(), 2);

                    if first.cmp(&secon).is_ne() {
                        return first.cmp(&secon);
                    }
                }
            }

            cmp
        });

        assert_eq!(res[0].0, "KKK23");
        assert_eq!(res[1].0, "AJJ94");
        assert_eq!(res[2].0, "A2223");
    }
}

fn calc_strength(input: &str) -> Win {
    // let mut char_vec = input.chars().into_iter().collect::<Vec<char>>();

    let mut groups = HashMap::new();
    for c in input.chars() {
        groups
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    if groups.len() == 1 {
        return Win::FIVE_OF_A_KIND;
    }

    if groups.len() == 2 {
        if groups.values().any(|v| *v == 4) {
            return Win::FOUR_OF_A_KIND;
        }

        return Win::FULL_HOUSE;
    }

    if groups.len() == 3 {
        if groups.values().any(|v| *v == 3) {
            return Win::THREE_OF_A_KIND;
        }

        return Win::TWO_PAIR;
    }

    if groups.len() == 4 {
        return Win::ONE_PAIR;
    }

    Win::HIGH_CARD
}

fn calc_strength_with_j(input: &str) -> Win {
    // let mut char_vec = input.chars().into_iter().collect::<Vec<char>>();

    let mut groups = HashMap::new();
    for c in input.chars() {
        groups
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let js = input.chars().filter(|c| *c == 'J').count();

    if groups.len() == 1 {
        return Win::FIVE_OF_A_KIND;
    }

    if groups.len() == 2 {
        if groups.values().any(|v| *v == 4) {
            if js == 1 || js == 4 {
                return Win::FIVE_OF_A_KIND
            }

            return Win::FOUR_OF_A_KIND;
        }

        if js == 2 || js == 3 {
            return Win::FIVE_OF_A_KIND;
        }

        return Win::FULL_HOUSE;
    }

    if groups.len() == 3 {
        let js = input.chars().filter(|c| *c == 'J').count();

        if groups.values().any(|v| *v == 3) {
            if js == 2 { return Win::FIVE_OF_A_KIND; }
            if js == 1 { return Win::FOUR_OF_A_KIND; }

            return Win::THREE_OF_A_KIND;
        }

        if js == 1 { return Win::FULL_HOUSE; }
        if js == 2 { return Win::FOUR_OF_A_KIND; }

        return Win::TWO_PAIR;
    }

    if groups.len() == 4 {
        println!("{} - {}", input, js);
        if js == 2  { return Win::THREE_OF_A_KIND; }
        if js == 1  { return Win::THREE_OF_A_KIND; }

        return Win::ONE_PAIR;
    }

    if js == 1 { return Win::ONE_PAIR; }

    Win::HIGH_CARD
}

fn map_strength_to_points(s: Win) -> i32 {
    match s {
        Win::FIVE_OF_A_KIND => 6,
        Win::FOUR_OF_A_KIND => 5,
        Win::FULL_HOUSE => 4,
        Win::THREE_OF_A_KIND => 3,
        Win::TWO_PAIR => 2,
        Win::ONE_PAIR => 1,
        Win::HIGH_CARD => 0,
    }
}

fn map_card_to_value(c: char, ver: i32) -> i32 {
    if ver == 1 {
        match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => panic!("Nope"),
        }
    } else {
        match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            'J' => -1,
            _ => panic!("Nope"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Win {
    FIVE_OF_A_KIND = 8,
    FOUR_OF_A_KIND = 7,
    FULL_HOUSE = 6,
    THREE_OF_A_KIND = 4,
    TWO_PAIR = 2,
    ONE_PAIR = 1,
    HIGH_CARD = 0,
}

fn calc_points_v1(l: &str) -> (&str, i32, i32) {
            let (cards, bet) = l.split_once(" ").unwrap();
            let bet = bet.parse::<i32>().unwrap();
            let points = map_strength_to_points(calc_strength(cards));
            (cards, bet, points)
}

fn calc_points_v2(l: &str) -> (&str, i32, i32) {
            let (cards, bet) = l.split_once(" ").unwrap();
            let bet = bet.parse::<i32>().unwrap();
            let points = map_strength_to_points(calc_strength_with_j(cards));
            (cards, bet, points)
}

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();

    let mut res = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| calc_points_v1(l))
        .collect::<Vec<(&str, i32, i32)>>();

    res.sort_by(|a, b| {
        let cmp = a.2.partial_cmp(&b.2).unwrap();

        if cmp.is_eq() {
            for idx in 0..a.0.chars().collect::<Vec<char>>().len() {
                let first = map_card_to_value(a.0.chars().nth(idx).unwrap(), 1);
                let secon = map_card_to_value(b.0.chars().nth(idx).unwrap(), 1);

                if first.cmp(&secon).is_ne() {
                    return first.cmp(&secon);
                }
            }
        }

        cmp
    });

    let mut result = 0;
    for (idx, val) in res.iter().enumerate() {
        result = result + (idx + 1) as i32 * val.1
    }

    println!("P1: {}", result);

    let mut res = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| calc_points_v2(l))
        .collect::<Vec<(&str, i32, i32)>>();

    res.sort_by(|a, b| {
        let cmp = a.2.partial_cmp(&b.2).unwrap();

        if cmp.is_eq() {
            for idx in 0..a.0.chars().collect::<Vec<char>>().len() {
                let first = map_card_to_value(a.0.chars().nth(idx).unwrap(), 2);
                let secon = map_card_to_value(b.0.chars().nth(idx).unwrap(), 2);

                if first.cmp(&secon).is_ne() {
                    return first.cmp(&secon);
                }
            }
        }

        cmp
    });

    let mut result = 0;
    for (idx, val) in res.iter().enumerate() {
        println!("{} {}", val.0, val.1);
        result = result + (idx + 1) as i32 * val.1
    }

    println!("P2: {}", result);
}
