# Rucksack Reorganization

## Part One

This problem allowed me to get some practice using `HashSet`. `HashSet`s are probably overkill for this problem but it was useful getting intersection functionality out of the box.

My approach for this part was the following:

1. Iterate over each line of the input.
2. Split each line at the mid point.
3. Create a `HashSet` for each split.
4. Find the intersection of the sets.
5. Convert the shared item into the priority value.

Because the prompt made it seem like there would only ever be a single common item in each half, this seemed like a pretty good approach without any edge cases.

First, I used `.split_at()` to get the two halves of the line and then created a `HashSet` of `chars` from each half.

```rust
for r in rucks.lines() {
    let (first, last) = r.split_at(r.len() / 2);

    let first: HashSet<char> = HashSet::from_iter(first.chars());
    let last: HashSet<char> = HashSet::from_iter(last.chars());
}
```

I'm not quite sure what the second generic for a `HashSet` is for, so I had to explicitly type my sets here, stating I only cared about characters. The `.from_iter()` initializer made creating a set from the iterator from `.chars()` simple.

All that was left to do was find the intersection.

```rust
let common = first.intersection(&last).next().unwrap();
```

Again, because the prompt made it seem like there would only ever be a single common item, I could just call `.next()` to get the first item in the interaction iterable.

To get the puzzle answer I needed to convert this common item to a priority value (a-z being 1-26 and A-Z being 27-52). I tried thinking of a clever way to do this, like converting the character to a number, but I just decided to do a lookup.

Unfortunately, creating constant `HashMap` to do this was not possible without manually writing out each entry ahead of time. I didn't want to take the time to do that so instead I create a slice of each character and wrote a helper to get the priority of the given item.

```rust
const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn convert_to_priority(item: &char) -> usize {
    let pos = PRIORITY.chars().position(|c| c == *item).unwrap();

    pos + 1
}
```

I summed up all the priority values and moved on to part two.
