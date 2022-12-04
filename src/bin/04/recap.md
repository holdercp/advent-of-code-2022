# Camp Cleanup

## Part One

This puzzle was a quick one. Once I had the data structured the way I wanted, getting the answer was simple.

First, I set up two custom structs to represent the assignments ("2-4", "6-8", etc.) and pairs of those assignments ("2-4,6-8").

```rust
struct Pair {
    first: Assignment,
    second: Assignment,
}

struct Assignment {
    start: u32,
    end: u32,
}
```

Next, I needed to convert the raw input into these types. To make this simple, I added a `build` associated function to each struct that handled this conversion.

```rust
impl Pair {
    fn build(p: &str) -> Pair {
        let assignments: Vec<&str> = p.split(",").collect();

        Pair {
            first: Assignment::build(assignments[0]),
            second: Assignment::build(assignments[1]),
        }
    }
}

impl Assignment {
    fn build(a: &str) -> Assignment {
        let sections: Vec<u32> = a.split("-").flat_map(|s| s.parse()).collect();

        Assignment {
            start: sections[0],
            end: sections[1],
        }
    }
}
```

I did this conversion by mapping over each line of the input.

```rust
let input = super::read_input();

input.lines().map(super::Pair::build);
```

Now that I had the data structured well, I needed to implement the core logic of determining if an assignment completely contains another. Since I had an `Assignment` struct ready to go, I added a method that took a reference to another `Assignment` and checked for the overlap.

```rust
impl Assignment {
    // ...

    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}
```

Then `.fold()`ed the `Pairs` into a total count of those where one assignment contains the other.

```rust
// ...
let contain_pairs: u32 = input.lines().map(super::Pair::build).fold(0, |count, p| {
    if p.first.contains(&p.second) || p.second.contains(&p.first) {
        return count + 1;
    }

    count
});

contain_pairs
```

Part one done!

## Part Two

I completed part two quickly because fortunately it was just adding functionality to part one. Instead of counting containing pairs, I needed to count overlapping pairs.

I implemented an `.overlaps()` method on my `Assignment` that checks for any overlap with another `Assignment`.

```rust
impl Assignment {
    // ...

    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains(other)
            || other.contains(self)
            || (self.start <= other.start && self.end >= other.start)
            || (other.start <= self.start && other.end >= self.start)
    }
}
```

The last step was to update my check in the core loop from part one.

```rust
let overlap_pairs: u32 = input.lines().map(super::Pair::build).fold(0, |count, p| {
    if p.first.overlaps(&p.second) {
        return count + 1;
    }

    count
});

overlap_pairs
```

Part two done!
