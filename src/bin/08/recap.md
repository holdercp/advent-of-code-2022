# Treetop Tree House

## Part One

I was set back a bit today because I misread the prompt. Nevertheless, I got to practice a lot of functional concepts in Rust with iterators.

First, I set up a simple struct to model the trees. This struct would contain the height, x, and y of the tree.

Then I parsed the input and returned a 2D vector of trees.

```rust
fn parse_input() -> Vec<Vec<Tree>> {
  let input =
      fs::read_to_string("src/bin/08/input.txt").expect("Should have been able to read the file");

  input
      .lines()
      .enumerate()
      .map(|(y, r)| {
          r.chars()
              .enumerate()
              .map(|(x, c)| {
                  let height = c.to_digit(10).unwrap();
                  Tree { height, x, y }
              })
              .collect::<Vec<Tree>>()
      })
      .collect()
}

struct Tree {
    height: u32,
    x: usize,
    y: usize,
}
```

The goal was to have an iterator of all the visible trees so I could just take the count of the iterator and have the answer. So in my head, I had something like this in mind:

```rust
let grid = super::parse_input();

grid.iter()
    .flat_map(|row| row.iter().filter(visible_trees))
    .count()
```

This is essentially what I ended up with but the meat of the solution is in the filter closure.

The filter's argument with be a double-borrowed tree. I'm not sure why filter closure's always borrow twice?

Within the filter I needed to do some slicing of the grid. Eventually, I wanted four slices of the grid (one for each visibility direction) that were copies, not borrows, because I needed to mutate them. I would then do some visibility checks against the current tree and the directional slices.

The first slices I took were the horizontal and vertical slices with the current tree being at the center of those axis.

Getting the horizontal slice was easy: get the row at the `y` index of the current tree.

```rust
let visible_trees_filter = |t: &&Tree| {
    let horizontal = &grid[t.y];
};
```

Getting the vertical slice was more awkward. I mapped over the grid and returned a new vector of every tree at the current tree's `x` index.

```rust
let visible_trees_filter = |t: &&Tree| {
    let vertical: &Vec<Tree> = &grid.iter().map(|rr| rr[t.x]).collect();
    let horizontal = &grid[t.y];
};
```

Now that I had the two broad directional slices, I needed to split them up into a up, down, left, right slice that didn't contain the current tree. This is where I needed to make copies because I needed to reverse the up and left slices. The problem consists of checking tree heights as you look *out from* the current tree, so it would make things simpler when checking the heights if each tree if they were in the order in which they should be checked.

```rust
let visible_trees_filter = |t: &&Tree| {
    let vertical: &Vec<Tree> = &grid.iter().map(|rr| rr[t.x]).collect();
    let horizontal = &grid[t.y];

    let up = &vertical[..t.y];
    let mut up = up.to_owned();
    up.reverse();

    let down = &vertical[t.y + 1..];
    let down = down.to_owned();

    let left = &horizontal[..t.x];
    let mut left = left.to_owned();
    left.reverse();

    let right = &horizontal[t.x + 1..];
    let right = right.to_owned();
};
```

The last step of this filter was to check if every tree in any of the directional slices was shorter than the current tree. If every tree in any one slice was shorter, then the tree is visible.

```rust
let visible_trees_filter = |t: &&Tree| {
    let vertical: &Vec<Tree> = &grid.iter().map(|rr| rr[t.x]).collect();
    let horizontal = &grid[t.y];

    let up = &vertical[..t.y];
    let mut up = up.to_owned();
    up.reverse();

    let down = &vertical[t.y + 1..];
    let down = down.to_owned();

    let left = &horizontal[..t.x];
    let mut left = left.to_owned();
    left.reverse();

    let right = &horizontal[t.x + 1..];
    let right = right.to_owned();

    let neighbors = [up, down, left, right];

    let is_visible = neighbors
        .iter()
        .any(|n| n.iter().all(|other| other.height < t.height));

    is_visible
};
```

With the filter done, I plugged it in to the original iterator chain and got the answer.

```rust
grid.iter()
    .flat_map(|r| r.iter().filter(visible_trees_filter))
    .count()
```

## Part Two

Part two was fairly easy to get after doing most of the slicing work in part one. Instead of filtering to visible trees, I needed to calculate the scenic score, which used the same directional slices.

I created a mapping closure to return an iterator of the scenic scores for each tree then got the max from the iterator.

```rust
grid.iter()
    .flat_map(|r| {
        r.iter().map(to_scenic_scores)
    })
    .max()
    .unwrap()
```

Let's look at the mapping closure.

I borrowed all the slicing code from part one's filter.

```rust
let to_scenic_scores = |t: &Tree| {
    let vertical: &Vec<Tree> = &grid.iter().map(|rr| rr[t.x]).collect();
    let horizontal = &grid[t.y];

    let up = &vertical[..t.y];
    let mut up = up.to_owned();
    up.reverse();

    let down = &vertical[t.y + 1..];
    let down = down.to_owned();

    let left = &horizontal[..t.x];
    let mut left = left.to_owned();
    left.reverse();

    let right = &horizontal[t.x + 1..];
    let right = right.to_owned();

    let neighbors = [up, down, left, right];  
};
```

I had my four directional slices. Now I needed to calculate the view distance of each slice. Fortunately, if the trees in the directional slices sorted to that the trees closest to the current tree is at the first index, then the view distance is just the index of the first tree that the same height or taller than the current tree.

With that, I just returned the position (index) of the first tree that had a height greater than or equal to the current tree's height. The `position` consumer returns an `Option` of the index. So I could match against that returned `Option` and get the correct distance in each of the match arms.

```rust
// to_scenic_score

// ...

let neighbors = [up, down, left, right];

let score = neighbors.iter().fold(1, |product, n| -> usize {
    let distance = n.iter().position(|other| other.height >= t.height);

    let d = match distance {
        Some(i) => i + 1,
        None => n.len(),
    };

    product * d
});

score
```

As you can see, I folded each distance into a single product for the total scenic score for the tree.

I found the max of all these and part two was done.
