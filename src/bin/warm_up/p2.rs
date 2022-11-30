pub fn solve() -> u32 {
    let depths = super::parse_input();
    let mut windows = depths.windows(3);
    let first = windows.next().unwrap();

    let (increases, _) = windows.fold((0 as u32, first), |mut acc, w| {
        let prev_sum: u32 = acc.1.iter().sum();
        let curr_sum: u32 = w.iter().sum();

        if curr_sum > prev_sum {
            acc.0 += 1;
        }

        acc.1 = w;

        acc
    });

    increases
}
