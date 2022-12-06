pub fn solve() -> usize {
    let buffer = super::read_input();

    let chars: Vec<char> = buffer.chars().collect();
    let mut marker = 0;

    for (i, set) in chars.windows(4).enumerate() {
        let mut set = set.to_vec();
        set.sort();

        let clone = set.clone();

        set.dedup();

        if set.len() == clone.len() {
            marker = i + 4;
            break;
        }
    }

    marker
}
