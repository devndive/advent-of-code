use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!(get_idx((0, 0), 3), 0);
        assert_eq!(get_idx((0, 1), 3), 1);
        assert_eq!(get_idx((0, 2), 3), 2);
        assert_eq!(get_idx((1, 0), 3), 3);
        assert_eq!(get_idx((1, 1), 3), 4);
        assert_eq!(get_idx((1, 2), 3), 5);
        assert_eq!(get_idx((2, 0), 3), 6);
        assert_eq!(get_idx((2, 1), 3), 7);
        assert_eq!(get_idx((2, 2), 3), 8);
    }

    #[test]
    fn test_three() {
        assert_eq!(get_pos(0, 3), (0, 0));
        assert_eq!(get_pos(1, 3), (0, 1));
        assert_eq!(get_pos(2, 3), (0, 2));
        assert_eq!(get_pos(3, 3), (1, 0));
        assert_eq!(get_pos(4, 3), (1, 1));
        assert_eq!(get_pos(5, 3), (1, 2));
        assert_eq!(get_pos(6, 3), (2, 0));
        assert_eq!(get_pos(7, 3), (2, 1));
        assert_eq!(get_pos(8, 3), (2, 2));
    }

    #[test]
    fn test_four() {
        let field = vec!['S', 'J', '|', 'F', 'L', 'J'];
        assert_eq!(
            get_possible_directions(0, 2, 3, &field),
            vec![(2, 'S'), (1, 'E')]
        );
        assert_eq!(get_possible_directions(2, 2, 3, &field), vec![(4, 'S')]);
        assert_eq!(get_possible_directions(1, 2, 3, &field), vec![]);
    }

    #[test]
    fn test_five() {
        assert_eq!(can_enter_2('|', '-', 'W'), false);
        assert_eq!(can_enter_2('|', 'L', 'S'), true);
        assert_eq!(can_enter_2('|', 'L', 'N'), false);
        assert_eq!(can_enter_2('7', 'F', 'E'), false);
    }
}

fn can_enter_2(cur_pos: char, next_pos: char, dir: char) -> bool {
    if cur_pos == 'S' {
        if dir == 'N' {
            return next_pos == '|' || next_pos == 'F' || next_pos == '7';
        } else if dir == 'S' {
            return next_pos == '|' || next_pos == 'J' || next_pos == 'L';
        } else if dir == 'E' {
            return next_pos == '-' || next_pos == 'J' || next_pos == '7';
        } else if dir == 'W' {
            return next_pos == '-' || next_pos == 'L' || next_pos == 'F';
        }
    }

    if cur_pos == '|' {
        if dir == 'S' {
            return next_pos == 'L' || next_pos == 'J' || next_pos == '|';
        } else if dir == 'N' {
            return next_pos == '7' || next_pos == 'F' || next_pos == '|';
        }
    }

    if cur_pos == '-' {
        if dir == 'E' {
            return next_pos == 'J' || next_pos == '7' || next_pos == '-';
        } else if dir == 'W' {
            return next_pos == 'L' || next_pos == 'F' || next_pos == '-';
        }
    }

    if cur_pos == 'L' {
        if dir == 'E' {
            return next_pos == 'J' || next_pos == '7' || next_pos == '-';
        } else if dir == 'N' {
            return next_pos == '|' || next_pos == '7' || next_pos == 'F';
        }
    }

    if cur_pos == 'J' {
        if dir == 'W' {
            return next_pos == 'F' || next_pos == 'L' || next_pos == '-';
        } else if dir == 'N' {
            return next_pos == '|' || next_pos == '7' || next_pos == 'F';
        }
    }

    if cur_pos == '7' {
        if dir == 'W' {
            return next_pos == 'F' || next_pos == 'L' || next_pos == '-';
        } else if dir == 'S' {
            return next_pos == '|' || next_pos == 'L' || next_pos == 'J';
        }
    }

    if cur_pos == 'F' {
        if dir == 'E' {
            return next_pos == 'J' || next_pos == '7' || next_pos == '-';
        } else if dir == 'S' {
            return next_pos == '|' || next_pos == 'L' || next_pos == 'J';
        }
    }

    false
}

fn get_pos(idx: usize, width: i32) -> (i32, i32) {
    let rem = idx as i32 % width;

    ((idx as i32 / width) as i32, rem as i32)
}

fn get_idx(pos: (i32, i32), width: i32) -> usize {
    (pos.0 * width as i32 + pos.1) as usize
}

fn get_possible_directions(
    idx: usize,
    width: i32,
    height: i32,
    field: &Vec<char>,
) -> Vec<(usize, char)> {
    let start_pos = get_pos(idx, width);

    let next_fields = vec![(-1, 0, 'N'), (1, 0, 'S'), (0, -1, 'W'), (0, 1, 'E')];

    next_fields
        .iter()
        .map(|n| {
            let next_r = start_pos.0 + n.0;
            let next_c = start_pos.1 + n.1;

            (next_r, next_c, n.2)
        })
        .filter(|n| {
            let in_bounds = n.0 >= 0 && n.0 < height && n.1 >= 0 && n.1 < width;
            // println!("{} {} - {} {}", n.0, height, n.1, width);
            if in_bounds {
                let idx = get_idx((n.0, n.1), width);
                return can_enter_2(field[get_idx(start_pos, width)], field[idx], n.2);
            }

            false
        })
        .map(|f| (get_idx((f.0, f.1), width), f.2))
        .collect::<Vec<(usize, char)>>()
}

fn has_at_least_one_zero_around(idx: usize, field: &Vec<char>, width: i32, height: i32) -> bool {
    let start_pos = get_pos(idx, width);
    // print!("{}: ", field[idx]);

    let around = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let no_zeros_around = around
        .iter()
        .map(|n| {
            let next_r = start_pos.0 + n.0;
            let next_c = start_pos.1 + n.1;

            (next_r, next_c)
        })
        .filter(|n| {
            let in_bounds = n.0 >= 0 && n.0 < height && n.1 >= 0 && n.1 < width;
            if in_bounds {
                let idx = get_idx((n.0, n.1), width);
                print!(" {}", field[idx]);
                return field[idx] == '0';
            }

            false
        })
        .count();

    // println!("");

    no_zeros_around != 0
}

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();

    let mut width = 0;

    let poschars = vec!['|', '-', 'L', 'J', '7', 'F', '.', 'S'];

    let mut field = vec![];
    input.split("\n").filter(|l| l.len() > 0).for_each(|l| {
        width = l.len();

        l.chars().for_each(|c| {
            if poschars.contains(&c) {
            field.push(c);
            }
        })
    });

    let height = input.split("\n").filter(|l| l.len() > 0).count();

    let start_idx = field.iter().position(|c| *c == 'S').unwrap();

    let width = width as i32;
    let height = height as i32;

    let mut visisted_and_distance = HashMap::new();
    let mut count = 0;

    let mut found_loop = false;
    let mut next_steps = get_possible_directions(start_idx, width, height, &field);

    while !found_loop {
        for steps in next_steps.iter() {
            match visisted_and_distance.insert(steps.0, count) {
                Some(_) => {
                    found_loop = true;
                    break;
                }
                None => {}
            }
        }

        count += 1;
        let next_steps_clone = next_steps.clone();
        next_steps = vec![];
        for steps in next_steps_clone.iter() {
            get_possible_directions(steps.0, width, height, &field)
                .iter()
                .filter(|p| !visisted_and_distance.contains_key(&p.0))
                .for_each(|p| {
                    next_steps.push((p.0, p.1));
                });
        }
    }

    println!("{:?}", count);

    for (idx, _) in field.clone().iter().enumerate() {
        if !visisted_and_distance.contains_key(&idx) {
            let pos = get_pos(idx, width);

            // go left till end or pipe
            let mut l_enclosed = false;
            for l_idx in (0..pos.1).rev() {
                if visisted_and_distance.contains_key(&get_idx((pos.0, l_idx), width)) {
                    l_enclosed = true;
                    break;
                }
            }

            let mut r_enclosed = false;
            for r_idx in pos.1..width {
                if visisted_and_distance.contains_key(&get_idx((pos.0, r_idx), width)) {
                    r_enclosed = true;
                }
            }

            let mut u_enclosed = false;
            for u_idx in (0..pos.0).rev() {
                if visisted_and_distance.contains_key(&get_idx((u_idx, pos.1), width)) {
                    u_enclosed = true;
                }
            }

            let mut d_enclosed = false;
            for d_idx in pos.0..height {
                if visisted_and_distance.contains_key(&get_idx((d_idx, pos.1), width)) {
                    d_enclosed = true;
                }
            }

            if l_enclosed && r_enclosed && u_enclosed && d_enclosed {
                field[idx] = 'I';
            } else {
                field[idx] = '0';
            }
        }
    }

    while field
        .clone()
        .iter()
        .enumerate()
        .filter(|f| {
            return *f.1 == 'I' && has_at_least_one_zero_around(f.0, &field.clone(), width, height);
        })
        .count()
        > 0
    {
        // println!("Are we doing this?");

        field = field
            .clone()
            .iter()
            .enumerate()
            .map(|f| {
                if *f.1 == 'I' && has_at_least_one_zero_around(f.0, &field, width, height) {
                    return '0';
                } else {
                    return *f.1;
                }
            })
            .collect::<Vec<char>>();
    }

    // check for squesies
    for (idx, _) in field.clone().iter().enumerate().filter(|f| *f.1 == 'I') {
        let pos = get_pos(idx, width);

        // go left till end or pipe
        let mut left_bot = false;
        let mut left_top = false;
        for l_idx in (0..pos.1).rev() {
            // let top_row = field[get_idx((pos.0 - 1, l_idx), width)];
            let mid_row = field[get_idx((pos.0 + 1, l_idx), width)];
            let bot_row = field[get_idx((pos.0, l_idx), width)];

            if !((mid_row == '-' || mid_row == 'J') && (bot_row == '7' || bot_row == '-')) {
                if mid_row == '0' || bot_row == '0' {
                    left_bot = true;
                    field[idx] = '0';
                    break;
                }
            }
        }

        if left_bot {
            break;
        }

        for l_idx in (0..pos.1).rev() {
            let top_row = field[get_idx((pos.0 - 1, l_idx), width)];
            let mid_row = field[get_idx((pos.0, l_idx), width)];
            // let bot_row = field[get_idx((pos.0 + 1, l_idx), width)];

            if !((mid_row == '-' || mid_row == '7') && (top_row == 'J' || top_row == '-')) {
                if mid_row == '0' || top_row == '0' {
                    left_top = true;
                    field[idx] = '0';
                    break;
                }
            }
        }

        if left_top {
            break;
        }

        let mut right_bot = false;
        let mut right_top = false;
        for r_idx in pos.1..width {
            // let top_row = field[get_idx((pos.0 - 1, l_idx), width)];
            let mid_row = field[get_idx((pos.0 + 1, r_idx), width)];
            let bot_row = field[get_idx((pos.0, r_idx), width)];

            if !((mid_row == '-' || mid_row == 'L') && (bot_row == 'F' || bot_row == '-')) {
                if mid_row == '0' || bot_row == '0' {
                    right_bot = true;
                    field[idx] = '0';
                    break;
                }
            }
        }

        if right_bot {
            break;
        }

        for r_idx in pos.1..width {
            let top_row = field[get_idx((pos.0 - 1, r_idx), width)];
            let mid_row = field[get_idx((pos.0, r_idx), width)];
            // let bot_row = field[get_idx((pos.0 + 1, l_idx), width)];

            if !((mid_row == '-' || mid_row == 'F') && (top_row == 'L' || top_row == '-')) {
                if mid_row == '0' || top_row == '0' {
                    right_top = true;
                    field[idx] = '0';
                    break;
                }
            }
        }

        if right_top {
            break;
        }

        let mut u_left = false;
        let mut u_right = false;
        for u_idx in (0..pos.0).rev() {
            let left_row = field[get_idx((u_idx, pos.1 - 1), width)];
            let mid_row = field[get_idx((u_idx, pos.1), width)];
            // let right_row = field[get_idx((u_idx.0, pos.1 + 1), width)];

            if !((mid_row == '|' || mid_row == 'F' || mid_row == 'L')
                && (left_row == 'J' || left_row == '|' || left_row == '7'))
            {
                if mid_row == '0' || left_row == '0' {
                    u_left = true;
                    field[idx] = '0';
                    break;
                }
            }
        }

        if u_left {
            break;
        }

        for u_idx in (0..pos.0).rev() {
            // let left_row = field[get_idx((u_idx, pox.1 - 1), width)];
            let mid_row = field[get_idx((u_idx, pos.1), width)];
            let right_row = field[get_idx((u_idx, pos.1 + 1), width)];

            if !((mid_row == 'J' || mid_row == '|' || mid_row == '7')
                && (right_row == 'F' || right_row == '|' || right_row == 'L'))
            {
                if mid_row == '0' || right_row == '0' {
                    u_right = true;
                    field[idx] = '0';
                    break;
                }
            }
        }

        if u_right {
            break;
        }

        let mut d_left = false;
        for d_idx in pos.0..height {
            let left_row = field[get_idx((d_idx, pos.1 - 1), width)];
            let mid_row = field[get_idx((d_idx, pos.1), width)];
            // let right_row = field[get_idx((d_idx.0, pos.1 + 1), width)];

            if !((mid_row == 'L' || mid_row == '|' || mid_row == 'F')
                && (left_row == '7' || left_row == '|' || left_row == 'J'))
            {
                if mid_row == '0' || left_row == '0' {
                    d_left = true;
                    field[idx] = '0';
                    break;
                }
            }
        }

        if d_left {
            break;
        }

        for d_idx in pos.0..height {
            // let left_row = field[get_idx((d_idx, pox.1 - 1), width)];
            let mid_row = field[get_idx((d_idx, pos.1), width)];
            let right_row = field[get_idx((d_idx, pos.1 + 1), width)];

            if !((mid_row == '7' || mid_row == '|' || mid_row == 'J')
                && (right_row == 'L' || right_row == '|' || right_row == 'F'))
            {
                if mid_row == '0' || right_row == '0' {
                    field[idx] = '0';
                    break;
                }
            }
        }
    }

    while field
        .clone()
        .iter()
        .enumerate()
        .filter(|f| {
            return *f.1 == 'I' && has_at_least_one_zero_around(f.0, &field.clone(), width, height);
        })
        .count()
        > 0
    {
        // println!("Are we doing this?");

        field = field
            .clone()
            .iter()
            .enumerate()
            .map(|f| {
                if *f.1 == 'I' && has_at_least_one_zero_around(f.0, &field, width, height) {
                    return '0';
                } else {
                    return *f.1;
                }
            })
            .collect::<Vec<char>>();
    }

            println!("");
    for (idx, f) in field.iter().enumerate() {
        print!("{}", f);

        if (idx + 1) as i32 % width == 0 {
            println!("");
        }
    }

    let is = field.iter().filter(|f| **f == 'I').count();
    println!("P2: {}", is);
}
