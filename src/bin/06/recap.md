# Tuning Trouble

## Part One

Today was a nice break and Rust's standard lib made this simple.

All I did was break the characters into `windows` of 4 and kept track of the index with `enumerate`. Then I iterated over the collection of windows and cloned each one. I then removed duplicates from the clone and checked if the lengths were different between the original and the clone. If they were not, that means each character was unique, so I added up the position plus the length of the windows.

```rust
pub fn solve() -> usize {
    let buffer = super::read_input();

    let chars: Vec<char> = buffer.chars().collect();
    let mut marker = 0;

    for (i, set) in chars.windows(4).enumerate() {
        let set = set.to_vec();

        let mut clone = set.clone();
        clone.sort();
        clone.dedup();

        if set.len() == clone.len() {
            marker = i + 4;
            break;
        }
    }

    marker
}
```

Easy peezy.

## Part Two

All that was needed was changing the window length to 14.

```rust
pub fn solve() -> usize {
    let buffer = super::read_input();

    let chars: Vec<char> = buffer.chars().collect();
    let mut marker = 0;

    for (i, set) in chars.windows(14).enumerate() {
        let set = set.to_vec();

        let mut clone = set.clone();
        clone.sort();
        clone.dedup();

        if set.len() == clone.len() {
            marker = i + 14;
            break;
        }
    }

    marker
}
```

Day 6 done.
