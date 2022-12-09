use std::collections::HashSet;

use super::Head;
use super::Motion;
use super::Segment;
use super::Tail;

pub fn solve() -> usize {
    let motions = super::parse_input();
    let mut head = Head { x: 0, y: 0 };
    let mut segment1 = Segment { x: 0, y: 0 };
    let mut segment2 = Segment { x: 0, y: 0 };
    let mut segment3 = Segment { x: 0, y: 0 };
    let mut segment4 = Segment { x: 0, y: 0 };
    let mut segment5 = Segment { x: 0, y: 0 };
    let mut segment6 = Segment { x: 0, y: 0 };
    let mut segment7 = Segment { x: 0, y: 0 };
    let mut segment8 = Segment { x: 0, y: 0 };
    let mut tail = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };

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
            segment1.step(&head);
            segment2.step(&segment1);
            segment3.step(&segment2);
            segment4.step(&segment3);
            segment5.step(&segment4);
            segment6.step(&segment5);
            segment7.step(&segment6);
            segment8.step(&segment7);
            tail.step(&segment8);
        }
    });

    tail.visited.len()
}
