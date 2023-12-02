use std::fs;

struct Draw {
    green: i32,
    red: i32,
    blue: i32,
}

fn parse_number(input: &str, color: &str) -> i32 {
    input
        .replace(color, "")
        .replace(" ", "")
        .parse::<i32>()
        .unwrap()
}

impl Draw {
    fn from(input: &str) -> Draw {
        let colors = input.split(",").collect::<Vec<&str>>();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for c in colors {
            if c.contains("red") {
                red = parse_number(c, "red");
            }

            if c.contains("green") {
                green = parse_number(c, "green");
            }

            if c.contains("blue") {
                blue = parse_number(c, "blue");
            }
        }

        return Draw { green, red, blue };
    }

    fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

struct Game {
    id: i32,
    draws: Vec<Draw>,
}

impl Game {
    fn from(line: &str) -> Game {
        let parts = line.split(": ").collect::<Vec<&str>>();

        let game_parts = parts[0].split(" ").collect::<Vec<&str>>();
        let id = game_parts[1].parse::<i32>().unwrap();

        let draws = parts[1]
            .split(";")
            .map(|raw_draw| Draw::from(raw_draw))
            .collect::<Vec<Draw>>();

        return Game { id, draws };
    }

    fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {
        self.draws.iter().all(|d| d.is_possible(red, green, blue))
    }

    fn power(&self) -> i32 {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for d in self.draws.iter() {
            if d.red > red {
                red = d.red;
            }

            if d.blue > blue {
                blue = d.blue;
            }

            if d.green > green {
                green = d.green;
            }
        }

        red * blue * green
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let games = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| Game::from(line))
        .collect::<Vec<Game>>();

    for game in games.iter() {
        println!("{}", game.id);
    }

    let sum = games
        .iter()
        .filter(|game| game.is_possible(12, 13, 14))
        .fold(0, |acc, g| acc + g.id);

    println!("Part 1: {sum}");

    let powers = games.iter().map(|g| g.power()).sum::<i32>();

    println!("Part 2: {powers}");
}
