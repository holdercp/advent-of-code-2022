use serde_json::Value;

use crate::parse_input2;

pub fn solve() -> i32 {
    let mut packets = parse_input2();

    let mut swap = false;

    loop {
        let mut index = 0;
        swap = false;

        loop {
            if index + 1 < packets.len() {
                let res = compare(&packets[index], &packets[index + 1]);

                match res {
                    State::Unsorted => {
                        let temp = packets[index].to_owned();
                        packets[index] = packets[index + 1].to_owned();
                        packets[index + 1] = temp;
                        swap = true;
                    }
                    _ => (),
                };

                index += 1;
            } else {
                break;
            }
        }

        if !swap {
            break;
        }
    }

    println!("{:?}", packets.iter().position(|v| {}));
    0
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

// fn is_sorted(p: &Pair) -> State {
//     let mut index = 0;

//     let left_packet = p.left.as_array().unwrap();
//     let right_packet = p.right.as_array().unwrap();

//     loop {
//         // We've reached the end of both packets
//         if index >= left_packet.len() && index >= right_packet.len() {
//             return State::Eq;
//         }

//         if index >= left_packet.len() {
//             return State::Sorted;
//         }

//         if index >= right_packet.len() {
//             return State::Unsorted;
//         }

//         let left = &p.left[index];
//         let right = &p.right[index];

//         if left.is_number() && right.is_number() {
//             let left = left.as_u64().unwrap();
//             let right = right.as_u64().unwrap();

//             if left < right {
//                 return State::Sorted;
//             } else if left > right {
//                 return State::Unsorted;
//             }

//             index += 1;
//             continue;
//         }

//         if left.is_number() {
//             let left = serde_json::to_value(vec![left]).unwrap();
//             let res = is_sorted_pair(&Pair {
//                 left,
//                 right: right.to_owned(),
//             });

//             match res {
//                 State::Eq => {
//                     index += 1;
//                     continue;
//                 }
//                 _ => {
//                     return res;
//                 }
//             };
//         }

//         if right.is_number() {
//             let right = serde_json::to_value(vec![right]).unwrap();
//             let res = is_sorted_pair(&Pair {
//                 left: left.to_owned(),
//                 right,
//             });

//             match res {
//                 State::Eq => {
//                     index += 1;
//                     continue;
//                 }
//                 _ => {
//                     return res;
//                 }
//             };
//         }

//         let res = is_sorted_pair(&Pair {
//             left: left.to_owned(),
//             right: right.to_owned(),
//         });

//         match res {
//             State::Eq => {
//                 index += 1;
//                 continue;
//             }
//             _ => {
//                 return res;
//             }
//         };
//     }
// }
