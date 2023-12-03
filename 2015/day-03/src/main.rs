use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve(">"), 2);
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point (i32, i32);

fn solve(input: &str) -> i32 {
    let mut point = Point(0, 0);

    let mut map: HashMap<Point, i32> = HashMap::new();
    map.insert(point, 1);

    for (_, direction) in input.chars().enumerate() {
        let _ = match direction {
            '>' => point.0 += 1,
            '<' => point.0 -= 1,
            '^' => point.1 += 1,
            'v' => point.1 -= 1,
            _ => panic!("Nope nope nope")
        };

        *map.entry(point).or_insert(1) += 1;
    }

    map.values().filter(|v| **v >= 1).count() as i32
}

fn solve2(input: &str) -> i32 {
    let mut point_santa = Point(0, 0);
    let mut point_robo = Point(0, 0);

    let mut map: HashMap<Point, i32> = HashMap::new();
    map.insert(point_santa, 2);

    for (pos, direction) in input.chars().enumerate() {
        if pos % 2 == 0 {
            match direction {
                '>' => point_robo.0 += 1,
                '<' => point_robo.0 -= 1,
                '^' => point_robo.1 += 1,
                'v' => point_robo.1 -= 1,
                _ => panic!("Nope nope nope")
            };

            *map.entry(point_robo).or_insert(1) += 1;
        } else {

            match direction {
                '>' => point_santa.0 += 1,
                '<' => point_santa.0 -= 1,
                '^' => point_santa.1 += 1,
                'v' => point_santa.1 -= 1,
                _ => panic!("Nope nope nope")
            };

            *map.entry(point_santa).or_insert(1) += 1;
        }
    }

    map.values().filter(|v| **v >= 1).count() as i32
}

fn main() {
    let input = std::fs::read_to_string("./src/input").unwrap();

    println!("1: {}", solve(&input));
    println!("2: {}", solve2(&input));
}
