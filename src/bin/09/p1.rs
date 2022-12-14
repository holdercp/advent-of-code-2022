use crate::Follower;
use crate::Knot;

use super::Head;
use super::Motion;
use super::Tail;

pub fn solve() -> usize {
    let motions = super::parse_input();
    let mut head = Head::new();
    let mut tail = Tail::new();

    tail.update_visited();

    motions.iter().for_each(|m| {
        head.r#move(m);

        let steps = match m {
            Motion::U(steps) => steps,
            Motion::D(steps) => steps,
            Motion::L(steps) => steps,
            Motion::R(steps) => steps,
        };

        for _ in 0..*steps {
            tail.step(&head);
        }
    });

    tail.visited.len()
}
