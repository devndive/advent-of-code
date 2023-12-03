use std::{fs, collections::{HashMap, HashSet}};

struct Field {
    grid: Vec<Vec<char>>,
}

impl Field {
    fn from(input: String) -> Field {
        let rows = input.split("\n").filter(|r| r.len() > 0).collect::<Vec<&str>>();

        let mut grid = vec![];

        for r in rows {
            let mut row = vec![];

            for c in r.chars().into_iter() {
                if c as u8 != 13 {
                    row.push(c);
                }
            }

            grid.push(row);
        }

        return Field {
            grid 
        }
    }

    fn get_all_part_numbers(&self) -> Vec<PartNumber> {
        let mut part_numbers: Vec<PartNumber> = vec![];

        for (row_number, row) in self.grid.iter().enumerate() {
            let mut indexes: Vec<Point> = vec![];
            let mut found_number = false;


            for (col_number, _col) in row.iter().enumerate() {
                let ch = self.grid[row_number][col_number]; 

                if ch.is_ascii_digit() {
                    indexes.push(Point { x: col_number, y: row_number, value: ch });
                    found_number = true;
                } else {
                    if found_number {
                        found_number = false;
                        let p = PartNumber { indexes };

                        if self.is_part_number(p.clone()) {
                            part_numbers.push(p);
                        }

                        indexes = vec![]
                    }
                }
            }

            // handle end of row numbers
            if found_number == true {
                        let p = PartNumber { indexes };
                        if self.is_part_number(p.clone()) {
                            part_numbers.push(p);
                        }
            }
        }

        part_numbers
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.grid[0].len() as i32 && y >= 0 && y < self.grid.len() as i32
    }

    fn is_part_number(&self, p: PartNumber) -> bool {

        let fields_around = [[-1, -1], [0, -1], [1, -1],
                             [-1,  0], [1,  0],
                             [-1,  1], [0,  1], [1,  1]];

        
        for i in p.indexes.iter() {
            for sur in fields_around.iter() {
                let x = i.x as i32 + sur[0];
                let y = i.y as i32 + sur[1];


                if self.is_in_bounds(x, y) {
                    let ch = self.grid[y as usize][x as usize];


                    if !ch.is_ascii_digit() && ch != '.' && ch as u8 != 13 {
                        return true
                    }
                }
            }
        }

        false
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    value: char
}

#[derive(Clone)]
struct PartNumber {
    indexes: Vec<Point>,
}

impl PartNumber {
    fn to_number(&self) -> i32 {
        let str = self.indexes.iter().fold(String::from(""), |acc, val| format!("{}{}", acc, val.value));

        str.parse::<i32>().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();

    let f = Field::from(input);
    let numbers = f.get_all_part_numbers();

    let mut sum = 0;
    for n in numbers.iter() {
        sum += n.to_number();
    }

    println!("{}", sum);
}
