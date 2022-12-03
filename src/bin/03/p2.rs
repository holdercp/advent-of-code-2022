use std::collections::HashSet;

pub fn solve() -> usize {
    let mut priority = 0;

    let input = super::read_input();
    let rucks: Vec<&str> = input.lines().collect();
    let ruck_chunks = rucks.chunks(3);

    for rc in ruck_chunks {
        let intersection = rc
            .iter()
            .map(|c| HashSet::from_iter(c.chars()))
            .reduce(|acc: HashSet<char>, set| {
                acc.intersection(&set).map(|c| c.to_owned()).collect()
            })
            .unwrap();

        let common = intersection.iter().next().unwrap();

        priority += super::convert_to_priority(common);
    }

    priority
}
