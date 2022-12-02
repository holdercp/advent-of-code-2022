pub fn solve() -> u32 {
    let cals = super::read_input();
    let mut sums: Vec<u32> = Vec::new();
    let mut sum = 0;

    for c in cals.lines() {
        if c.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            let c_num: u32 = c.parse().expect("Failed to parse");
            sum += c_num;
        }
    }

    sums.sort();

    let top = &sums[sums.len() - 3..];
    top.iter().sum()
}
