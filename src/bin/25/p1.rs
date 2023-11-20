const BASE: i64 = 5;

pub fn solve() -> String {
    let fuel_requirements = super::read_input();

    let snafu_to_decimal = |sum: i64, snafu: (usize, char)| {
        let decimal: i64 = match snafu.1 {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("bad snafu digit"),
        };

        sum + decimal * (5_i64.pow(u32::try_from(snafu.0).unwrap()))
    };

    let sum_snafu = |sum: i64, r: &str| {
        let converted_snafu = r.chars().rev().enumerate().fold(0, snafu_to_decimal);

        sum + converted_snafu
    };

    let decimal_sum = fuel_requirements.lines().fold(0, sum_snafu);

    let snafu_sum = decimal_into_snafu(&decimal_sum, false);

    snafu_sum.iter().cloned().collect()
}

fn decimal_into_snafu(decimal: &i64, carry: bool) -> Vec<char> {
    if decimal == &0 {
        return vec![];
    }

    let res = decimal / BASE;
    let mut rem = decimal % BASE;

    if carry {
        rem += 1;
    }

    let should_carry = rem > 2;

    let dig = if should_carry { rem - BASE } else { rem };

    let place = match dig {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("bad remainder"),
    };

    let mut rest = decimal_into_snafu(&res, should_carry);
    rest.push(place);

    rest
}
