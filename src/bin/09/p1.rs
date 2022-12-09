// use std::collections::HashSet;

// use super::Head;
// use super::Tail;

// pub fn solve() -> usize {
//     let motions = super::parse_input();
//     let mut head = Head { x: 0, y: 0 };
//     let mut tail = Tail {
//         x: 0,
//         y: 0,
//         visited: HashSet::new(),
//     };

//     tail.update_visited();

//     motions.iter().for_each(|m| {
//         head.r#move(m);
//         tail.follow(&head);
//     });

//     tail.visited.len()
// }
