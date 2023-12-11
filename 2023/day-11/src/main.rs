use core::panic;
use std::{collections::HashMap, ops::Index};

struct Field {
    width: i32,
    height: i32,

    tiles: Vec<char>,
}

impl Field {
    fn from(input: &str) -> Self {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut tiles = vec![];

        input.split("\n").filter(|l| l.len() > 0).for_each(|l| {
            width = l.len() as i32;
            height += 1;

            l.chars().for_each(|c| {
                tiles.push(c);
            })
        });

        Self {
            width,
            height,
            tiles,
        }
    }

    fn get_pos(&self, idx: usize) -> (i32, i32) {
        let rem = idx as i32 % self.width;

        ((idx as i32 / self.width) as i32, rem as i32)
    }

    fn get_idx(&self, pos: (i32, i32)) -> usize {
        (pos.0 * self.width as i32 + pos.1) as usize
    }

    fn find_expanding_col(&self, expansion_count: i32) -> Self {
        let mut cols_to_expand = vec![];
        for c in 0..self.width {
            let mut should_expand = true;
            for r in 0..self.height {
                let pos = self.get_idx((r, c));

                if self.tiles[pos] != '.' {
                    should_expand = false;
                }
            }

            if should_expand {
                cols_to_expand.push(c);
            }
        }

        println!("{cols_to_expand:?}");

        let mut new_tiles = vec![];
        let new_width = self.width + cols_to_expand.len() as i32;

        for (idx, tile) in self.tiles.iter().enumerate() {
            let pos = self.get_pos(idx);
            new_tiles.push(*tile);

            if cols_to_expand.contains(&pos.1) {
                for _ in 0..expansion_count {
                    new_tiles.push('.');
                }
            }
        }

        Self {
            tiles: new_tiles,
            width: new_width,
            height: self.height,
        }
    }

    fn find_expanding_row(&self, expansion_count: i32) -> Self {
        let mut rows_to_expand = vec![];
        for r in 0..self.height {
            let mut should_expand = true;
            for c in 0..self.width {
                let pos = self.get_idx((r, c));

                if self.tiles[pos] != '.' {
                    should_expand = false;
                }
            }

            if should_expand {
                rows_to_expand.push(r);
            }
        }

        println!("{rows_to_expand:?}");

        let mut new_tiles = vec![];
        let new_height = self.height + rows_to_expand.len() as i32;

        for (idx, tile) in self.tiles.iter().enumerate() {
            let pos = self.get_pos(idx);

            if rows_to_expand.contains(&pos.0) {
                for _ in 0..expansion_count {
                    for _ in 0..self.width {
                        new_tiles.push('.');
                    }
                }

                rows_to_expand.retain(|r| *r != pos.0);
            }

            new_tiles.push(*tile);
        }

        Self {
            tiles: new_tiles,
            width: self.width,
            height: new_height,
        }
    }

    fn get_planets(&self) -> Vec<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|t| *t.1 == '#')
            .map(|t| t.0)
            .collect::<Vec<usize>>()
    }

    fn get_next_paths(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        let around = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        around
            .iter()
            .map(|a| (a.0 + pos.0, a.1 + pos.1))
            .filter(|a| a.0 >= 0 && a.0 < self.height && a.1 >= 0 && a.1 < self.width)
            .collect::<Vec<(i32, i32)>>()
    }

    fn from_a_to_b(&self, a: usize, b: usize) -> i32 {
        let mut visited = HashMap::new();
        visited.insert(self.get_pos(a), 1);

        let target_pos = self.get_pos(b);
        let mut distance = 1;

        let found = false;
        let mut to_check = self.get_next_paths(self.get_pos(a));

        while !found {
            let v_clone = visited.clone();
            let clone_to_check = to_check.clone();
            to_check = vec![];

            // println!("To check - {}", clone_to_check.len());

            for check in clone_to_check {
                if self.get_idx(check) == b {
                    // found = true;
                    return distance;
                } else {
                    visited.insert(check, distance);
                }

                let check_to_target_distance = ((check.0 - target_pos.0).pow(2) as f32 + (check.1 - target_pos.1).pow(2) as f32).sqrt();
                for steps in self.get_next_paths(check) {
                    let step_to_target_distance = ((steps.0 - target_pos.0).pow(2) as f32 + (steps.1 - target_pos.1).pow(2) as f32).sqrt();

                    // println!("({:?} - {:?}): {} <= {} = {}", steps, target_pos,step_to_target_distance, check_to_target_distance, step_to_target_distance < check_to_target_distance);

                    /*
                    if steps == target_pos {
                        found = true;
                        return distance;
                    }
                    */

                    if !visited.contains_key(&steps) && (step_to_target_distance <= check_to_target_distance) {
                        visited.insert(steps, distance);
                        to_check.push(steps);
                    }
                }
            }

            distance += 1;

            // self.print_v(&visited);
            /*
            if distance == 9 {
                found = true;
            }
            */
        }

        panic!("We should never be here");
    }

    fn print(&self) {
        for (idx, f) in self.tiles.iter().enumerate() {
            print!("{}", f);

            if (idx + 1) as i32 % self.width == 0 {
                println!("");
            }
        }
    }

    fn print_v(&self, visisted: &HashMap<(i32, i32), i32>) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // println!("");
        for (idx, f) in self.tiles.iter().enumerate() {
            if visisted.contains_key(&self.get_pos(idx)) {
                print!("v");
            } else {
                print!("{}", f);
            }

            if (idx + 1) as i32 % self.width == 0 {
                println!("");
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/sample").unwrap();
    let f = Field::from(&input);

    let f = f.find_expanding_col(10).find_expanding_row(10);

    let planets = f.get_planets();

    /*
    let distance = f.from_a_to_b(planets[1], planets[2]);
    println!("2 ({}) - 3 ({}) => {} (9)", planets[1], planets[2], distance);

    let distance = f.from_a_to_b(planets[4], planets[8]);
    println!("5 - 9 => {} (9)", distance);

    let distance = f.from_a_to_b(planets[0], planets[6]);
    println!("1 - 7 => {} (15)", distance);

    let distance = f.from_a_to_b(planets[2], planets[5]);
    println!("3 - 6 => {}", distance);

    let distance = f.from_a_to_b(planets[7], planets[8]);
    println!("8 - 9 => {}", distance);
    */
    
    let mut distances = vec![];
    for (idx, p) in planets.iter().enumerate() {
        println!("Processing planet {}", idx + 1);
        for target in planets.iter().skip(idx + 1) {
            // print!("{} -> {}", idx + 1, planets.iter().position(|p| p == target).unwrap() +1);
            let distance = f.from_a_to_b(*p, *target);
            //print!(" = {}", distance);
            distances.push(distance);
            //println!("");
        }
    }

    println!("{}", distances.iter().sum::<i32>());
}
