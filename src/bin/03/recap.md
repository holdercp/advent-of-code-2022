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

## Part Two

I wrestled with types a lot in this part. Learning about iterators, what they return, and keeping the chain going with the correct type did not come easy. Fortunately, the Rust error output is really helpful and I got to a working solution.

My approach for this part was:

1. Group three lines together.
2. Map over each line in a group and convert it to a set.
3. Reduce the sets down to a single intersection by finding the intersection of the first item and the second, then finding the intersection of that result with the last set.

I discovered the `.chunks()` method that creates an iterator of groups of a given size. That made step 1 and 2 simple.

```rust
    let input = super::read_input();
    let rucks: Vec<&str> = input.lines().collect();
    let ruck_chunks = rucks.chunks(3);

    for rc in ruck_chunks {
        rc.iter().map(|c| HashSet::from_iter(c.chars()))
    }
```

Then things got a little tricky with the types. Now that I had an iterator of sets of chars, I needed to get the intersection of each set with the next one and return the resulting intersection as a new set so I could do it again for the last item.

`.reduce()` seemed perfect for this because it uses the first item in the iterator as the initial accumulator value.

The issue I ran into was returning a `HashSet<char>` with each iteration. Typing the accumulator explicitly informed the `.collect()` method what it should collect to but it was referencing the `chars` in the resulting set instead of having owned `chars`. I then mapped over the result of the intersection and returned a `char` after calling `.to_owned()`. I'm not sure if this is the best way but it worked.

```rust
for rc in ruck_chunks {
    let intersection = rc
        .iter()
        .map(|c| HashSet::from_iter(c.chars()))
        .reduce(|acc: HashSet<char>, set| {
            acc.intersection(&set).map(|c| c.to_owned()).collect()
        })
        .unwrap();
}
```

The final steps were the same as part one: get the first item in the resulting intersection and convert it to a priority value.

Done.
