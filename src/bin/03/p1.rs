use std::collections::HashSet;

pub fn solve() -> usize {
    let rucks = super::read_input();

    let mut priority = 0;

    for r in rucks.lines() {
        let (first, last) = r.split_at(r.len() / 2);

        let first: HashSet<char> = HashSet::from_iter(first.chars());
        let last: HashSet<char> = HashSet::from_iter(last.chars());

        let common = first.intersection(&last).next().unwrap();
        priority += super::convert_to_priority(common);
    }

    priority
}
