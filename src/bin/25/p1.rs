pub fn solve() -> i32 {
    let fuel_requirements = super::read_input();

    let snafu_to_decimal = |sum: i32, snafu: (usize, char)| {
        let decimal: i32 = match snafu.1 {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("bad snafu digit"),
        };

        sum + decimal * (5_i32.pow(u32::try_from(snafu.0).unwrap()))
    };

    fuel_requirements.lines().fold(0, |sum, r| {
        let converted_snafu = r.chars().rev().enumerate().fold(0, snafu_to_decimal);

        sum + converted_snafu
    })
}
