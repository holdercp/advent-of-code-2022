# Supply Sacks

## Part One

Today was a bear. The puzzle wasn't the difficult part. Parsing the input was the challenge today.

After splitting up the "supply" (the representation of the crates in stacks) input and the procedure input, I read in the lines that pertained only to the supply. First, I popped of the line with the column numbers, and then I reversed the iterator so the bottom of the stacks would be read in first.

I initialized an empty vector for the 9 stacks and broke up each line into chuncks of 4.

I broke the line into chunks of 4 because each column has a length of 3 plus the column separator marked by an empty space. I couldn't split on empty spaces because some stacks are shorter than others, meaning they will have empty "columns". Spliting into chunks of 4 means I could always check to see if the "column" had content. If it did, I had a crate, which I pushed into the appropriate stack vector.

```rust
fn build_supply(raw: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = raw.lines().collect();

    // We don't care about the stack num from the input
    lines.pop();

    lines.reverse();

    let mut supply: Vec<Vec<char>> = vec![Vec::new(); 9];

    for line in lines {
        let cols: Vec<char> = line.chars().collect();
        let chunks = cols.chunks(4);

        for (i, chunk) in chunks.enumerate() {
            if chunk[0] == '[' {
                supply[i].push(chunk[1]);
            }
        }
    }

    supply
}
```

To parse the procedure, I created a vector of tuples. Each value representing a "move", "from", and "to" in that order.

```rust
fn build_procedure(raw: &str) -> Vec<(usize, usize, usize)> {
    let mut steps: Vec<(usize, usize, usize)> = Vec::new();

    for line in raw.lines() {
        let nums: Vec<usize> = line.split_whitespace().flat_map(|w| w.parse()).collect();

        steps.push((nums[0], nums[1], nums[2]));
    }

    steps
}
```

Here's my final `parse_input()`:

```rust
fn parse_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input =
        fs::read_to_string("src/bin/05/input.txt").expect("Should have been able to read the file");

    let input: Vec<&str> = input.split("\n\n").collect();

    let supply = build_supply(input[0]);
    let procedure = build_procedure(input[1]);

    (supply, procedure)
}
```

For the actual work, I iterated over each step in the procedure and mutated the stacks based on the step values. Because each move value meant taking a crate off a stack one at a time, I could've popped each crate but instead I took a whole slice off the top of the stack and then reversed that slice before appending it into the new stack.

```rust
let (mut supply, procedure) = super::parse_input();

for step in procedure {
    let stack = &mut supply[step.1 - 1];

    let mut taken: Vec<char> = stack.drain(stack.len() - step.0..).collect();
    taken.reverse();

    supply[step.2 - 1].append(&mut taken);
}
```

The last step was to concat the top crate on each stack and return the new string.

```rust
let mut top = String::new();
for mut stack in supply {
    let c = stack.pop().unwrap();
    top.push(c);
}

top
```

Part one done.

## Part Two

The second part was easy. Because the order of the sliced crates needed to stay intact, I just didn't reverse the slice like I did in part one.

```rust
let (mut supply, procedure) = super::parse_input();

for step in procedure {
    let stack = &mut supply[step.1 - 1];

    let mut taken: Vec<char> = stack.drain(stack.len() - step.0..).collect();
    // taken.reverse();

    supply[step.2 - 1].append(&mut taken);
}
```

Part two done.
