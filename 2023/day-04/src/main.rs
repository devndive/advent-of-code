use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53");
        assert_eq!(points(card), 8);
    }

    #[test]
    fn test_two() {
        let card = Card::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(points(card), 2);
    }

    #[test]
    fn test_three() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53");
        assert_eq!(points2(&card), 4);
    }

    #[test]
    fn test_four() {
        let card = Card::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(points2(&card), 2);
    }
}

struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>
}

impl Card {
    fn from(input: &str) -> Self {
        let parts = input.split(":").collect::<Vec<&str>>();
        let id = parts[0].replace("Card", "").trim().parse::<i32>().unwrap();

        let number_groups = parts[1].split(" | ").collect::<Vec<&str>>();

        let winning_numbers = number_groups[0]
            .trim()
            .split(" ")
            .filter(|raw| raw.len() > 0)
            .map(|raw| raw.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let numbers = number_groups[1]
            .trim()
            .split(" ")
            .filter(|raw| raw.len() > 0)
            .map(|raw| raw.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Card {
            id,
            winning_numbers,
            numbers
        }
    }
}

fn points(card: Card) -> i32 {
    let winners = card.numbers
        .into_iter()
        .filter(|n| {
            let contains = card.winning_numbers.contains(n);
            contains
        })
        .collect::<Vec<i32>>();
    
    if winners.len() >= 1 {
        2_i32.pow(winners.len() as u32 - 1)
    } else {
        0
    }
}

fn points2(card: &Card) -> i32 {
    card.numbers
        .iter()
        .filter(|n| {
            let contains = card.winning_numbers.contains(n);
            contains
            
        })
        .collect::<Vec<&i32>>()
        .len() as i32
}

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();

    let cards = input.split("\n")
            .filter(|l| l.len() > 0)
            .map(|l| Card::from(l));

    let points = cards.clone().into_iter().map(|c| points(c)).sum::<i32>();

    let mut pile: HashMap<i32, i32> = HashMap::new();
    cards.clone().for_each(|c| { pile.insert(c.id, 1); });

    for card in cards.clone() {
        let points = points2(&card);
        println!("{}: {}", card.id, points);

        if points > 0 {
            let clone = pile.clone();
            let current_card_count = clone.get(&card.id).unwrap();

            for idx in 1..points+1 {
                pile.entry(idx + card.id).and_modify(|e| *e += *current_card_count);
            }
        }
    }

    println!("P1: {}", points);
    println!("P2: {}", pile.iter().map(|e| e.1).sum::<i32>());
}
