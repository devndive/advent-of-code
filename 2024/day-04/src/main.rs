#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_from() {
        let f = Field::from("123\n456\n789");

        assert_eq!(f.height, 3);
        assert_eq!(f.width, 3);
    }

    #[test]
    fn test_index_to_pos() {
        let f = Field::from("123\n456\n789");

        assert_eq!(f.index_to_pos(0), (0, 0));
        assert_eq!(f.index_to_pos(1), (1, 0));
        assert_eq!(f.index_to_pos(2), (2, 0));
        assert_eq!(f.index_to_pos(3), (0, 1));
        assert_eq!(f.index_to_pos(4), (1, 1));
        assert_eq!(f.index_to_pos(5), (2, 1));
        assert_eq!(f.index_to_pos(6), (0, 2));
        assert_eq!(f.index_to_pos(7), (1, 2));
        assert_eq!(f.index_to_pos(8), (2, 2));
    }

    #[test]
    fn pos_to_index() {
        let f = Field::from("123\n456\n789");

        assert_eq!(f.pos_to_index((0, 0)), 0);
        assert_eq!(f.pos_to_index((1, 0)), 1);
        assert_eq!(f.pos_to_index((2, 0)), 2);
        assert_eq!(f.pos_to_index((0, 1)), 3);
        assert_eq!(f.pos_to_index((1, 1)), 4);
        assert_eq!(f.pos_to_index((2, 1)), 5);
        assert_eq!(f.pos_to_index((0, 2)), 6);
        assert_eq!(f.pos_to_index((1, 2)), 7);
        assert_eq!(f.pos_to_index((2, 2)), 8);
    }

    #[test]
    fn test_neighbours() {
        let f = Field::from("123\n456\n789");

        assert_eq!(f.neighbours(0), vec![1, 3, 4]);
        assert_eq!(f.neighbours(1), vec![0, 2, 3, 4, 5]);
        assert_eq!(f.neighbours(2), vec![1, 4, 5]);
        assert_eq!(f.neighbours(3), vec![0, 1, 4, 6, 7]);
        assert_eq!(f.neighbours(4), vec![0, 1, 2, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn test_field() {
        let f = Field::from(
            "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
        );

        assert_eq!(f.tiles[f.pos_to_index((2, 0))], 'X');
        assert_eq!(f.tiles[f.pos_to_index((1, 0))], '.');
        assert_eq!(f.tiles[f.pos_to_index((3, 0))], '.');
        assert_eq!(f.tiles[f.pos_to_index((1, 1))], 'S');
        assert_eq!(f.tiles[f.pos_to_index((2, 1))], 'A');
        assert_eq!(f.tiles[f.pos_to_index((3, 1))], 'M');
        assert_eq!(f.tiles[f.pos_to_index((4, 1))], 'X');
        assert_eq!(f.tiles[f.pos_to_index((3, 1))], 'M');
        assert_eq!(f.tiles[f.pos_to_index((2, 1))], 'A');
        assert_eq!(f.tiles[f.pos_to_index((1, 1))], 'S');
    }
}

use std::env;
use std::fs;

struct Field {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Field {
    fn from(input: &str) -> Self {
        let mut field = Field {
            tiles: vec![],
            width: 0,
            height: 0,
        };

        let lines = input.lines();
        field.height = lines.count();

        input.lines().for_each(|l| {
            field.width = l.len();
            for c in l.chars() {
                field.tiles.push(c);
            }
        });

        field
    }

    fn index_to_pos(self: &Self, idx: usize) -> (usize, usize) {
        let y = idx / self.width;
        let x = idx - y * self.width;

        (x, y)
    }

    fn pos_to_index(self: &Self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn neighbours(self: &Self, idx: usize) -> Vec<usize> {
        let mut n = vec![];

        let surround = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            // (0, 0), this is the item itself
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let pos = self.index_to_pos(idx);

        for sur in surround {
            let x = pos.0 as i32 + sur.0;
            let y = pos.1 as i32 + sur.1;

            if pos.0 as i32 + sur.0 >= 0
                && pos.0 as i32 + sur.0 < self.width as i32
                && pos.1 as i32 + sur.1 >= 0
                && pos.1 as i32 + sur.1 < self.height as i32
            {
                n.push(self.pos_to_index((x as usize, y as usize)));
            }
        }

        return n;
    }

    fn is_in_bounds(self: &Self, pos: (usize, usize), sur: (i32, i32)) -> bool {
        let x = pos.0 as i32 + sur.0;
        let y = pos.1 as i32 + sur.1;

        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn is_in_bounds_2(self: &Self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.width as i32 && pos.1 >= 0 && pos.1 < self.height as i32
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let field = Field::from(&contents);

    let mut xmas_counter = 0;
    let mut x_mas_counter = 0;
    for (idx, tile) in field.tiles.iter().enumerate() {
        if *tile == 'X' {
            println!("Found X, looking for neighbors now");

            let surround = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                // (0, 0), this is the item itself
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];

            let pos = field.index_to_pos(idx);

            for sur in surround {
                if field.is_in_bounds(pos, sur) {
                    let m_pos = ((pos.0 as i32 + sur.0), (pos.1 as i32 + sur.1));
                    let a_pos = ((pos.0 as i32 + sur.0 * 2), (pos.1 as i32 + sur.1 * 2));
                    let s_pos = ((pos.0 as i32 + sur.0 * 3), (pos.1 as i32 + sur.1 * 3));

                    if field.is_in_bounds_2(m_pos)
                        && field.is_in_bounds_2(a_pos)
                        && field.is_in_bounds_2(s_pos)
                        && field.tiles[field.pos_to_index((m_pos.0 as usize, m_pos.1 as usize))]
                            == 'M'
                        && field.tiles[field.pos_to_index((a_pos.0 as usize, a_pos.1 as usize))]
                            == 'A'
                        && field.tiles[field.pos_to_index((s_pos.0 as usize, s_pos.1 as usize))]
                            == 'S'
                    {
                        xmas_counter += 1;
                    }
                }
            }
        }

        if *tile == 'A' {
            let pairs = [((-1, -1), (1, 1)), ((1, -1), (-1, 1))];

            let pos = field.index_to_pos(idx);
            println!("A: {:?}: ", pos);
            let mut mas_counter = 0;
            for p in pairs.iter() {
                let first = ((pos.0 as i32 + p.0 .0), (pos.1 as i32 + p.0 .1));
                let second = ((pos.0 as i32 + p.1 .0), (pos.1 as i32 + p.1 .1));

                if field.is_in_bounds_2(first)
                    && field.is_in_bounds_2(second)
                    && field.tiles[field.pos_to_index((first.0 as usize, first.1 as usize))] == 'M'
                    && field.tiles[field.pos_to_index((second.0 as usize, second.1 as usize))]
                        == 'S'
                {
                    mas_counter += 1;
                }

                let third = ((pos.0 as i32 + p.1 .0), (pos.1 as i32 + p.1 .1));
                let fourth = ((pos.0 as i32 + p.0 .0), (pos.1 as i32 + p.0 .1));

                if field.is_in_bounds_2(third)
                    && field.is_in_bounds_2(fourth)
                    && field.tiles[field.pos_to_index((third.0 as usize, third.1 as usize))] == 'M'
                    && field.tiles[field.pos_to_index((fourth.0 as usize, fourth.1 as usize))]
                        == 'S'
                {
                    mas_counter += 1;
                }
            }
            if mas_counter >= 2 {
                x_mas_counter += 1;
            }
        }
    }

    println!("{}", xmas_counter);
    println!("X-MAS: {}", x_mas_counter);
}
