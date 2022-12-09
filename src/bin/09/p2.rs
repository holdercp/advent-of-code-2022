use std::collections::HashSet;

use super::Head;
use super::Tail;

pub fn solve() -> usize {
    let motions = super::parse_input();
    let mut head = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail1 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail2 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail3 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail4 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail5 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail6 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail7 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail8 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };
    let mut tail9 = Tail {
        x: 0,
        y: 0,
        visited: HashSet::new(),
    };

    tail9.update_visited();

    motions.iter().for_each(|m| {
        head.r#move(m);
        tail1.follow(&head);
        tail2.follow(&tail1);
        tail3.follow(&tail2);
        tail4.follow(&tail3);
        tail5.follow(&tail4);
        tail6.follow(&tail5);
        tail7.follow(&tail6);
        tail8.follow(&tail7);
        tail9.follow(&tail8);
    });

    tail9.visited.len()
}
