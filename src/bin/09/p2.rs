use crate::Follower;
use crate::Knot;

use super::Head;
use super::Motion;
use super::Segment;
use super::Tail;

pub fn solve() -> usize {
    let motions = super::parse_input();
    let mut head = Head::new();
    let mut segment1 = Segment::new();
    let mut segment2 = Segment::new();
    let mut segment3 = Segment::new();
    let mut segment4 = Segment::new();
    let mut segment5 = Segment::new();
    let mut segment6 = Segment::new();
    let mut segment7 = Segment::new();
    let mut segment8 = Segment::new();
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
