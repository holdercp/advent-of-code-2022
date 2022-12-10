# Cathode-Ray Tube

## Part One

This was a fun problem! The piece that made my solutions simple was parsing the raw input into the "cycles".

To do that, I returned a vector of integers. Each index represented a cycle in the CPU. The cycle was a `0` if the operation was `noop` or it was in the middle of "processing" the `addx`. All I did was insert an extra `0` entry for any `addx` operation.

```rust
fn parse_input() -> Vec<i32> {
    let input =
        fs::read_to_string("src/bin/10/input.txt").expect("Should have been able to read the file");

    let mut instructions = Vec::new();

    input.lines().for_each(|l| {
        if l.starts_with("addx") {
            l.split_whitespace()
                .flat_map(|s| s.parse::<i32>())
                .for_each(|v| {
                    instructions.push(0);
                    instructions.push(v);
                })
        } else {
            instructions.push(0);
        }
    });

    instructions
}
```

Solving the puzzle was simple with this data. I looped over the instructions vector, collected the signal strength at the appropriate cycles, and updated the register.

To get the answer, I summed up the signal strength entries.

```rust
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
```

## Part Two

The output for part two was great! Although it took me a couple of tries to actually input the correct sequence.

For this part, I set up the `display` as a 40x6 slice of `.` chars (off pixel).

I then looped over the instructions and turned on the pixel if the cycle and register points matched up.

The tricky part was getting the correct position on the display. I used a modulo operation for the horizontal position but a helper function to get the vertical position.

```rust
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
            display[row(&cycle)][pos] = '#';
        }

        register += value;
    }

    display.iter().map(|r| r.to_vec()).collect()
}

fn row(cycle: &usize) -> usize {
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

```

Part two done.
