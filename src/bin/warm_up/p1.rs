pub fn solve() -> u32 {
    let depths = super::parse_input();
    let mut increases: u32 = 0;
    let mut prev = &depths[0];

    for d in &depths[1..] {
        if d > prev {
            increases += 1;
        }
        prev = d;
    }

    increases
}
