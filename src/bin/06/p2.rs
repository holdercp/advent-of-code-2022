pub fn solve() -> usize {
    let buffer = super::read_input();

    let chars: Vec<char> = buffer.chars().collect();
    let mut marker = 0;

    for (i, set) in chars.windows(14).enumerate() {
        let set = set.to_vec();

        let mut clone = set.clone();
        clone.sort();
        clone.dedup();

        if set.len() == clone.len() {
            marker = i + 14;
            break;
        }
    }

    marker
}
