use crate::{Direction, Instruction, TileValue};

pub fn solve() -> u32 {
    let ((start, map), instructions) = super::parse_input();

    let mut dir = Direction::Right;
    let mut tile = map.get(&start).unwrap();

    for instruction in instructions {
        match instruction {
            Instruction::RotateLeft => {
                dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            Instruction::RotateRight => {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            Instruction::Move(n) => {
                for _ in 0..n {
                    let next_point = tile.neighbors.get(&dir).unwrap();
                    let next_tile = map.get(next_point).unwrap();

                    if let TileValue::Wall = next_tile.value {
                        break;
                    }

                    tile = next_tile;
                }
            }
        };
    }

    let dir_value = match dir {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
    };

    (1_000 * (tile.location.y as u32 + 1)) + (4 * (tile.location.x as u32 + 1)) + dir_value
}
