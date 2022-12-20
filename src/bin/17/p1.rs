use std::collections::HashSet;

use crate::{Point, Rock, Wind};

pub fn solve() -> u32 {
    let wind = super::parse_input();
    let shape_order = super::get_shape_arr();

    const LEFT_WALL: u32 = 0;
    const RIGHT_WALL: u32 = 8;
    const FLOOR: u32 = 0;
    const LIMIT: u32 = 2022;
    const HEIGHT_OFFSET: u32 = 1;

    let mut rocks_fallen: u32 = 0;
    let mut wind_index = 0;
    let mut tower_height = HEIGHT_OFFSET;

    let mut chamber: HashSet<Point> = HashSet::new();

    loop {
        let mut rock = Rock::new(&shape_order[rocks_fallen as usize % 5], &tower_height);

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
                tower_height = chamber.iter().map(|p| p.y).max().unwrap() + HEIGHT_OFFSET;
                break;
            } else {
                rock.points = fallen;
            }
        }
    }

    tower_height - HEIGHT_OFFSET
}
