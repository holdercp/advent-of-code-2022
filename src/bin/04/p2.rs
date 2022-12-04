pub fn solve() -> u32 {
    let input = super::read_input();

    let overlap_pairs: u32 = input.lines().map(super::Pair::build).fold(0, |count, p| {
        if p.first.overlaps(&p.second) || p.second.overlaps(&p.first) {
            return count + 1;
        }

        count
    });

    overlap_pairs
}
