struct Race {
    time: i64,
    distance: i64,
    number_of_ways_to_win: i64,
}

fn main() {
    /*
    let mut races = [
        Race{ time: 7, distance: 9, number_of_ways_to_win: 0 },
        Race{ time: 15, distance: 40, number_of_ways_to_win: 0 },
        Race{ time: 30, distance: 200, number_of_ways_to_win: 0 }
    ];
    */

    /*
    let mut races = [
        Race{ time: 53, distance: 313, number_of_ways_to_win: 0 },
        Race{ time: 89, distance: 1090, number_of_ways_to_win: 0 },
        Race{ time: 76, distance: 1214, number_of_ways_to_win: 0 },
        Race{ time: 98, distance: 1201, number_of_ways_to_win: 0 },
    ];
    */

    /*
    let mut races = [
        Race{ time: 71530, distance: 940200, number_of_ways_to_win: 0 },
    ];
    */

    let mut races = [
        Race{ time: 53897698, distance: 313109012141201, number_of_ways_to_win: 0 },
    ];

    for race in races.iter_mut() {
        let mut winner_count = 0;
        for hold_time in 1..race.time {

            let speed = hold_time;

            let remaining_time = race.time - hold_time;
            let distance_travelled = speed * remaining_time;

            if distance_travelled > race.distance {
                winner_count += 1;
            }

            //println!("{speed} * {remaining_time} = {distance_travelled}");
        }

        race.number_of_ways_to_win = winner_count;
    }

    let p1 = races.iter().fold(1, |acc, val| acc * val.number_of_ways_to_win);
    println!("P1: {p1}");
}
