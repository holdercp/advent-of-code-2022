use std::collections::HashSet;

pub fn solve() -> usize {
    let mut priority = 0;

    let input = super::read_input();
    let rucks: Vec<&str> = input.lines().collect();
    let ruck_chunks = rucks.chunks(3);

    for r in ruck_chunks {
        let first: HashSet<char> = HashSet::from_iter(r[0].chars());
        let second: HashSet<char> = HashSet::from_iter(r[1].chars());
        let last: HashSet<char> = HashSet::from_iter(r[2].chars());

        let common: HashSet<char> = first.intersection(&second).map(|c| c.to_owned()).collect();
        let common: &char = common.intersection(&last).next().unwrap();

        priority += super::convert_to_priority(common);
    }

    priority
}
