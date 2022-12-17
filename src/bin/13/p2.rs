use std::cmp::Ordering;

use serde_json::Value;

use crate::parse_input2;

pub fn solve() -> usize {
    let mut packets = parse_input2();

    loop {
        let mut index = 0;
        let mut swap = false;

        loop {
            if index + 1 < packets.len() {
                let res = compare(&packets[index], &packets[index + 1]);

                if let State::Unsorted = res {
                    let temp = packets[index].to_owned();
                    packets[index] = packets[index + 1].to_owned();
                    packets[index + 1] = temp;
                    swap = true;
                }

                index += 1;
            } else {
                break;
            }
        }

        if !swap {
            break;
        }
    }

    (find_divider_packet(&packets, 6) + 1) * (find_divider_packet(&packets, 2) + 1)
}

fn find_divider_packet(packets: &[Value], divider: u64) -> usize {
    packets
        .iter()
        .position(|p| {
            if let Value::Array(outer) = p {
                if outer.len() == 1 {
                    if let Value::Array(inner) = &outer[0] {
                        if inner.len() == 1 {
                            if let Some(n) = inner[0].as_u64() {
                                return n == divider;
                            }
                        }
                    }
                }
            }

            false
        })
        .unwrap()
}

fn compare(a: &Value, b: &Value) -> State {
    let mut index = 0;

    let left_packet = a.as_array().unwrap();
    let right_packet = b.as_array().unwrap();

    loop {
        // We've reached the end of both packets
        if index >= left_packet.len() && index >= right_packet.len() {
            return State::Eq;
        }

        if index >= left_packet.len() {
            return State::Sorted;
        }

        if index >= right_packet.len() {
            return State::Unsorted;
        }

        let left = &left_packet[index];
        let right = &right_packet[index];

        if left.is_number() && right.is_number() {
            let left = left.as_u64().unwrap();
            let right = right.as_u64().unwrap();

            match left.cmp(&right) {
                Ordering::Less => return State::Sorted,
                Ordering::Greater => return State::Unsorted,
                Ordering::Equal => {
                    index += 1;
                    continue;
                }
            };
        }

        if left.is_number() {
            let left = serde_json::to_value(vec![left]).unwrap();
            let res = compare(&left, right);

            match res {
                State::Eq => {
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
            let res = compare(left, &right);

            match res {
                State::Eq => {
                    index += 1;
                    continue;
                }
                _ => {
                    return res;
                }
            };
        }

        let res = compare(left, right);

        match res {
            State::Eq => {
                index += 1;
                continue;
            }
            _ => {
                return res;
            }
        };
    }
}

#[derive(Debug)]
enum State {
    Sorted,
    Unsorted,
    Eq,
}
