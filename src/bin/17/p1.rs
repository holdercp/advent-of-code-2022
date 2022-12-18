use std::collections::HashSet;

use crate::{JetPattern, Line, Point, Rock};

pub fn solve() -> String {
    let patterns = super::parse_input();

    const LEFT_BOUND: u32 = 0;
    const RIGHT_BOUND: u32 = 6;

    let mut floor: HashSet<Point> = (0..7).map(|n| Point { x: n, y: 0 }).collect();

    let highest_point = floor.iter().max_by_key(|p| p.y).unwrap();

    let mut falling_rock = Line::new(Point {
        x: RIGHT_BOUND,
        y: highest_point.y + 3,
    });

    let mut fallen_rocks: u32 = 1;
    let mut movement = Movement::Jet;
    let mut pattern_index: usize = 0;

    loop {
        match movement {
            Movement::Jet => {
                let pattern = patterns.get(pattern_index % patterns.len()).unwrap();

                match pattern {
                    JetPattern::Left => falling_rock.shift_left(LEFT_BOUND),
                    JetPattern::Right => falling_rock.shift_right(RIGHT_BOUND),
                };

                pattern_index += 1;
                movement = Movement::toggle(movement);
            }
            Movement::Fall => {
                if let Err(points) = falling_rock.fall(&floor) {
                    // TODO: Update floor
                    floor = falling_rock = Line::new(start);
                    fallen_rocks += 1;
                };
            }
        };
    }
}

enum Movement {
    Jet,
    Fall,
}

impl Movement {
    fn toggle(movement: Movement) -> Movement {
        match movement {
            Movement::Fall => Movement::Jet,
            Movement::Jet => Movement::Fall,
        }
    }
}
