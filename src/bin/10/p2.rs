pub fn solve() -> Vec<Vec<char>> {
    let instructions = super::parse_input();

    let mut display = [['.'; 40]; 6];
    let mut register: i32 = 1;

    for (cycle, value) in instructions.iter().enumerate() {
        let (l, c, r) = (
            (register - 1) as usize,
            register as usize,
            (register + 1) as usize,
        );
        let pos = cycle % 40;

        if pos == l || pos == c || pos == r {
            display[get_display_row(&cycle)][cycle % 40] = '#';
        }

        register += value;
    }

    display.iter().map(|r| r.to_vec()).collect()
}

fn get_display_row(cycle: &usize) -> usize {
    if cycle < &40 {
        0
    } else if cycle < &80 {
        1
    } else if cycle < &120 {
        2
    } else if cycle < &160 {
        3
    } else if cycle < &200 {
        4
    } else {
        5
    }
}
