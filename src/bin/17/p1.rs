use std::collections::HashSet;

use crate::{Point, Rock, Wind};

pub fn solve() -> i64 {
    let wind = super::parse_input();
    let shape_order = super::get_shape_arr();

    const LEFT_WALL: i64 = -1;
    const RIGHT_WALL: i64 = 7;
    const FLOOR: i64 = -1;
    const LIMIT: u64 = 2022;

    let mut rocks_fallen: u64 = 0;
    let mut wind_index = 0;
    let mut tower_height = 0;

    let mut chamber: HashSet<Point> = HashSet::new();

    loop {
        let mut rock = Rock::new(
            &shape_order[rocks_fallen as usize % shape_order.len()],
            tower_height,
        );

        rocks_fallen += 1;

        if rocks_fallen > LIMIT {
            break;
        }

        loop {
            match wind[wind_index % wind.len()] {
                Wind::Left => {
                    let shifted = rock.shift_left();

                    if !shifted
                        .iter()
                        .any(|p| p.x == LEFT_WALL || chamber.get(p).is_some())
                    {
                        rock.points = shifted;
                    }
                }
                Wind::Right => {
                    let shifted = rock.shift_right();

                    if !shifted
                        .iter()
                        .any(|p| p.x == RIGHT_WALL || chamber.get(p).is_some())
                    {
                        rock.points = shifted;
                    }
                }
            };

            wind_index += 1;

            let fallen = rock.fall();

            if fallen
                .iter()
                .any(|p| p.y == FLOOR || chamber.get(p).is_some())
            {
                chamber.extend(rock.points.into_iter());
                tower_height = chamber.iter().map(|p| p.y).max().unwrap() + 1;
                break;
            } else {
                rock.points = fallen;
            }
        }
    }

    tower_height
}
