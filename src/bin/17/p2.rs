use std::collections::HashMap;

use crate::{Point, Rock, Wind};

pub fn solve() -> i64 {
    let winds = super::parse_input();
    let shape_order = super::get_shape_arr();

    const LEFT_WALL: i64 = -1;
    const RIGHT_WALL: i64 = 7;
    const FLOOR: i64 = -1;
    const TARGET: u64 = 1_000_000_000_000;

    let mut cycled = false;
    let mut cycle_remainder = 0;

    let mut rocks_fallen: u64 = 0;
    let mut wind_index = 0;
    let mut tower_height: i64 = 0;

    let mut chamber: Vec<Point> = Vec::new();

    let mut states: HashMap<(Vec<Point>, usize, usize), (i64, u64)> = HashMap::new();

    loop {
        let shape_index = rocks_fallen as usize % shape_order.len();

        let mut rock = Rock::new(
            &shape_order[shape_index],
            &chamber.iter().map(|p| p.y).max().unwrap_or(-1) + 1,
        );

        if cycled && rocks_fallen == cycle_remainder {
            return tower_height - 1;
        }

        rocks_fallen += 1;

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
                tower_height = chamber.iter().map(|p| p.y).max().unwrap() + 1;
                break;
            } else {
                rock.points = fallen;
            }
        }

        if !cycled {
            let normalized_floor = normalize_chamber_state(&chamber);

            let state =
                states.get_key_value(&(normalized_floor, shape_index, wind_index % winds.len()));

            match state {
                Some(((floor, _, _), (prev_height, prev_rocks_fallen))) => {
                    // We've cycled
                    let height_delta = tower_height - prev_height;
                    let rocks_fallen_delta = rocks_fallen - prev_rocks_fallen;

                    let rocks_remaining = TARGET - rocks_fallen;
                    let cycles = rocks_remaining / rocks_fallen_delta;
                    let height_gained = cycles as i64 * height_delta + tower_height;

                    cycle_remainder =
                        rocks_remaining - (rocks_fallen_delta * cycles) + rocks_fallen;

                    tower_height = height_gained;
                    chamber = denormalize_floor(floor.to_vec(), tower_height);

                    cycled = true;
                    continue;
                }
                None => {
                    let normalized_floor = normalize_chamber_state(&chamber);

                    states.insert(
                        (normalized_floor, shape_index, wind_index % winds.len()),
                        (tower_height, rocks_fallen),
                    );
                }
            };
        }
    }
}

fn normalize_chamber_state(chamber: &[Point]) -> Vec<Point> {
    let max_ys = chamber.iter().fold(
        [Point { x: 0, y: 0 }; 7],
        |mut acc: [Point; 7], point: &Point| {
            if acc[point.x as usize].y < point.y {
                acc[point.x as usize] = *point;
            }

            acc
        },
    );

    let min_y = max_ys.iter().min_by_key(|p| p.y).unwrap().y;

    let mut floor: Vec<Point> = chamber.iter().filter(|p| p.y >= min_y).cloned().collect();

    let offset: i64 = 0_i64.abs_diff(min_y).try_into().unwrap();
    floor.iter_mut().for_each(|p| {
        p.y -= offset;
    });

    floor
}

fn denormalize_floor(mut floor: Vec<Point>, tower_height: i64) -> Vec<Point> {
    let max_y = floor.iter().max_by_key(|p| p.y).unwrap().y;

    let offset: i64 = max_y.abs_diff(tower_height).try_into().unwrap();
    floor.iter_mut().for_each(|p| {
        p.y += offset;
    });

    floor
}
