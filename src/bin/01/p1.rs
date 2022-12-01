pub fn solve() -> u32 {
    let cals = super::parse_input();
    let mut greatest = 0;
    let mut sum = 0;

    for c in cals.lines() {
        if c.is_empty() {
            if sum > greatest {
                greatest = sum;
            }

            sum = 0;
        } else {
            let c_num: u32 = c.parse().expect("Failed to parse");
            sum += c_num;
        }
    }

    greatest
}
