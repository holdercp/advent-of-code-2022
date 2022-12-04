pub fn solve() -> u32 {
    let input = super::read_input();

    let contain_pairs: u32 = input.lines().map(super::Pair::build).fold(0, |count, p| {
        if p.first.contains(&p.second) || p.second.contains(&p.first) {
            return count + 1;
        }

        count
    });

    contain_pairs
}
