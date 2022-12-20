use std::collections::HashMap;

use crate::{Point, Rock, Wind};

pub fn solve() -> u64 {
    let winds = super::parse_input();
    let shape_order = super::get_shape_arr();

    const LEFT_WALL: u64 = 0;
    const RIGHT_WALL: u64 = 8;
    const FLOOR: u64 = 0;
    const LIMIT: u64 = 1_000_000_000_000;
    const HEIGHT_OFFSET: u64 = 1;

    let mut rocks_fallen: u64 = 0;
    let mut wind_index = 0;
    let mut tower_height = HEIGHT_OFFSET;

    let mut chamber: Vec<Point> = Vec::new();

    // let mut index_state = HashMap::new();

    loop {
        let shape_index = rocks_fallen as usize % shape_order.len();

        let mut rock = Rock::new(&shape_order[shape_index], &tower_height);

        rocks_fallen += 1;

        if rocks_fallen > LIMIT {
            break;
        }

        loop {
            match &winds[wind_index % winds.len()] {
                Wind::Left => {
                    let shifted = rock.shift_left();

                    if !shifted
                        .iter()
                        .any(|p| p.x == LEFT_WALL || chamber.contains(p))
                    {
                        rock.points = shifted;
                    }
                }
                Wind::Right => {
                    let shifted = rock.shift_right();

                    if !shifted
                        .iter()
                        .any(|p| p.x == RIGHT_WALL || chamber.contains(p))
                    {
                        rock.points = shifted;
                    }
                }
            };

            wind_index += 1;

            let fallen = rock.fall();

            if fallen.iter().any(|p| p.y == FLOOR || chamber.contains(p)) {
                chamber.extend(rock.points.into_iter());
                tower_height = chamber.iter().map(|p| p.y).max().unwrap() + HEIGHT_OFFSET;
                break;
            } else {
                rock.points = fallen;
            }
        }

        // let res = states.get(&(shape_index, wind_index - 1));

        // match res {
        //     Some((prev_chamber, prev_height)) => {
        //         if prev_chamber.len() % 2 == 0 {
        //             let chamber
        //         } else {
        //             states.insert((shape_index, wind_index), (chamber, tower_height));
        //         }
        //     }
        //     None => states.insert((shape_index, wind_index), (chamber, tower_height)),
        // };

        if chamber.len() % 2 == 0 {
            let (left, right) = chamber.split_at(chamber.len() / 2);

            let y_offset = left[0].y.abs_diff(right[0].y);

            let right: Vec<Point> = right
                .iter()
                .map(|p| Point {
                    x: p.x,
                    y: p.y - y_offset,
                })
                .collect();

            let eq = left
                .iter()
                .enumerate()
                .all(|(i, p)| p.x == right[i].x && p.y == right[i].y);

            if eq {
                println!("{} {}", tower_height, rocks_fallen);
            }
        }

        println!("{}", tower_height);
    }

    tower_height - HEIGHT_OFFSET
}
