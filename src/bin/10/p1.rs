pub fn solve() -> i32 {
    let instructions = super::parse_input();

    let mut signal_strengths = Vec::new();
    let mut register = 1;

    for (cycle, value) in instructions.iter().enumerate() {
        match cycle + 1 {
            20 => signal_strengths.push(20 * register),
            60 => signal_strengths.push(60 * register),
            100 => signal_strengths.push(100 * register),
            140 => signal_strengths.push(140 * register),
            180 => signal_strengths.push(180 * register),
            220 => signal_strengths.push(220 * register),
            _ => (),
        };

        register += value;
    }

    signal_strengths.iter().sum()
}
