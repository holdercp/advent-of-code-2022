# No Space Left On Device

## Part One

This one was a fun one. It finally felt like Advent of Code. I also made a bad assumption with this one too, which cost me a lot of time to debugging.

My first thought was to build out a tree structure of the file system after parsing the input. As I thought about it more, I realized if I was parsing the input to build the tree then I might as well just solve the problem as I went instead of doing two passes: one to build the tree, and then one to traverse it and solve the problem.

This was a risk because part two could've thrown a wrench in this plan but luckily it didn't

On to the code.

My approach was this:

1. Read the input line by line
2. Check for two things:
   1. A `cd` command
   2. A file

Those are the only two things we need to solve the puzzle. I'll break down why.

The `cd` command let's us keep track of where we are in the file system. So if we store the current directory we're in and the one's we've been to to end up here, then we essentially have the pwd.

To do this I set up a vector as a stack that I could push/pop entries to/off as I `cd`'ed around.

```rust
let mut history: Vec<String> = Vec::new();

for line in input.lines() {
    if line.starts_with("$ cd ") {
        let arg = &line[5..];

        if arg == ".." {
            history.pop();
        } else {
            let mut dir = String::new();
            for d in history.clone() {
                dir.push_str(&d);
            }
            dir.push_str(arg);
            history.push(dir);
        }
    }
}
```

Here, I'm getting the arg of the `cd` command. If it's `..`, I pop my last entry off the history, otherwise I construct the "pwd" and push it onto the history.

I constructed the full path because I hit a bug in my first attempt. I assumed all directory names would be unique; they were not. However, the paths would be unique, so that's what I ended up storing as the directory identifier.

The next thing I cared about from the input was when a file was found. When this happened, I parsed the size into an integer and added it to a `sums` hash map that had an entry for every directory with a file in it. Since I had the full path in `history`, I looped through it, adding the new file size to any sum that existed for the directory entry already.

```rust
let mut history: Vec<String> = Vec::new();
let mut sums: HashMap<String, u32> = HashMap::new();

for line in input.lines() {
    if line.starts_with("$ cd ") {
        let arg = &line[5..];

        if arg == ".." {
            history.pop();
        } else {
            let mut dir = String::new();
            for d in history.clone() {
                dir.push_str(&d);
            }
            dir.push_str(arg);
            history.push(dir);
        }
    }

    if line.starts_with(|c: char| c.is_numeric()) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let size: u32 = split[0].parse().unwrap();

        for dir in history.clone() {
            let sum = sums.entry(dir).or_insert(0);
            *sum += size;
        }
    }
}
```

All that was left to do was get the some of the directories in `sums`.

```rust
let input = super::read_input();

let mut history: Vec<String> = Vec::new();
let mut sums: HashMap<String, u32> = HashMap::new();

for line in input.lines() {
    if line.starts_with("$ cd ") {
        let arg = &line[5..];

        if arg == ".." {
            history.pop();
        } else {
            let mut dir = String::new();
            for d in history.clone() {
                dir.push_str(&d);
            }
            dir.push_str(arg);
            history.push(dir);
        }
    }

    if line.starts_with(|c: char| c.is_numeric()) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let size: u32 = split[0].parse().unwrap();

        for dir in history.clone() {
            let sum = sums.entry(dir).or_insert(0);
            *sum += size;
        }
    }
}

sums.into_values().filter(|v| v <= &100000).sum()
```

On to part two.

## Part Two

This is where I held my breath. Luckily, I didn't need to manipulate the file strucuture.

All I needed to do here was some extra math on top of the solution to part one.

```rust
const TOTAL_SIZE: u32 = 70_000_000;
const REQUIRED_SIZE: u32 = 30_000_000;

let input = super::read_input();

let mut history: Vec<String> = Vec::new();
let mut sums: HashMap<String, u32> = HashMap::new();

for line in input.lines() {
    if line.starts_with("$ cd ") {
        let arg = &line[5..];

        if arg == ".." {
            history.pop();
        } else {
            let mut dir = String::new();
            for d in history.clone() {
                dir.push_str(&d);
            }
            dir.push_str(arg);
            history.push(dir);
        }
    }

    if line.starts_with(|c: char| c.is_numeric()) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let size: u32 = split[0].parse().unwrap();

        for dir in history.clone() {
            let sum = sums.entry(dir).or_insert(0);
            *sum += size;
        }
    }
}

let used_space = sums.get("/").unwrap();
let free_space = TOTAL_SIZE - used_space;
let needed_space = REQUIRED_SIZE - free_space;

*sums.values().filter(|v| v >= &&needed_space).min().unwrap()

```

Part two done!
