pub fn solve() -> Vec<i32> {
    let fuel_requirements = super::read_input();

    let sums = fuel_requirements
        .lines()
        .map(|r| {
            r.chars().rev().enumerate().fold(0, |sum, c| {
                let decimal: i32 = match c.1 {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!("bad snafu digit"),
                };

                sum + decimal * (5_i32.pow(u32::try_from(c.0).unwrap()))
            })
        })
        .collect();

    sums
}
