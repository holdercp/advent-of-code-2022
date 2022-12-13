use crate::{parse_input, Pair};

pub fn solve() -> usize {
    let pairs = parse_input();
    let mut sum = 0;

    for (i, p) in pairs.iter().enumerate() {
        let res = is_sorted_pair(p);

        match res {
            State::Sorted => {
                sum += i + 1;
            }
            _ => (),
        };
    }

    sum
}

fn is_sorted_pair(p: &Pair) -> State {
    let mut index = 0;

    let left_packet = p.left.as_array().unwrap();
    let right_packet = p.right.as_array().unwrap();

    loop {
        // We've reached the end of both packets
        if index >= left_packet.len() && index >= right_packet.len() {
            return State::Indeterminate;
        }

        if index >= left_packet.len() {
            return State::Sorted;
        }

        if index >= right_packet.len() {
            return State::Unsorted;
        }

        let left = &p.left[index];
        let right = &p.right[index];

        if left.is_number() && right.is_number() {
            let left = left.as_u64().unwrap();
            let right = right.as_u64().unwrap();

            if left < right {
                return State::Sorted;
            } else if left > right {
                return State::Unsorted;
            }

            index += 1;
            continue;
        }

        if left.is_number() {
            let left = serde_json::to_value(vec![left]).unwrap();
            let res = is_sorted_pair(&Pair {
                left,
                right: right.to_owned(),
            });

            match res {
                State::Indeterminate => {
                    index += 1;
                    continue;
                }
                _ => {
                    return res;
                }
            };
        }

        if right.is_number() {
            let right = serde_json::to_value(vec![right]).unwrap();
            let res = is_sorted_pair(&Pair {
                left: left.to_owned(),
                right,
            });

            match res {
                State::Indeterminate => {
                    index += 1;
                    continue;
                }
                _ => {
                    return res;
                }
            };
        }

        let res = is_sorted_pair(&Pair {
            left: left.to_owned(),
            right: right.to_owned(),
        });

        match res {
            State::Indeterminate => {
                index += 1;
                continue;
            }
            _ => {
                return res;
            }
        };
    }
}

enum State {
    Sorted,
    Unsorted,
    Indeterminate,
}
