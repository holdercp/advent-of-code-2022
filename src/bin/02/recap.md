# Rock Paper Scissors

## Part One

Today I started to dig into the functional features of Rust, as well as defining enums and structs.

My overall approach was this:

1. Iterate over each line of the input
2. Transform each line into a `Round`
3. Calculate the score for each `Round` and add it to a total score

### Setup

The first step I took was defining a `Shape` enum so I could more easily keep track of whether the input was a rock, paper, or scissors.

```rust
enum Shape {
    Rock,
    Paper,
    Scissors,
}
```

Next up, I defined a `Round` as struct with an `opponent_shape` property and a `my_shape` property:

```rust
struct Round {
    opponent_shape: Shape,
    my_shape: Shape,
}
```

I needed a function to turn the raw input ("A", "B", "C", etc.) into `Shape`s and then `Round`s. This was pretty simple with Rust's `match` construct since both "columns" of the input represented shapes.

```rust
fn create_shape(shape_raw: &str) -> Shape {
    match shape_raw {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("bad shape"),
    }
}
```

Lastly, I created a closure to create a `Round` for each line of the input. This was my first step into Rust's functional features. With this closure, I could `.map()` over each line and transform it into a `Round`.

```rust
let guide = super::read_input();

let create_round = |line: &str| {
    let cols: Vec<&str> = line.split_whitespace().collect();
    Round {
        opponent_shape: create_shape(cols[0]),
        my_shape: create_shape(cols[1]),
    }
};

guide.lines().map(create_round) // I get an iterator of Rounds here
```

### Solving

Cool. Now we have our input nicely structured. All that remained was to calculate the score of each round.

The prompt describes two scores that are summed to get the total score for the round:

1. The score of the shape I play (represented by the `Round.my_shape`)
2. The score of the outcome (win, lose, draw)

I created individual functions to handle both of these tasks.

```rust
fn calc_shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn calc_outcome_score(round: &Round) -> i32 {
    match round.opponent_shape {
        Shape::Rock => match round.my_shape {
            Shape::Rock => 3,
            Shape::Paper => 6,
            Shape::Scissors => 0,
        },
        Shape::Paper => match round.my_shape {
            Shape::Rock => 0,
            Shape::Paper => 3,
            Shape::Scissors => 6,
        },
        Shape::Scissors => match round.my_shape {
            Shape::Rock => 6,
            Shape::Paper => 0,
            Shape::Scissors => 3,
        },
    }
}
```

The first function was easy. Take in a `Shape` reference, `match` on it, and return the correct score.

The second function wasn't too bad but the nested `match`es got a little unweildy. I do, however, much prefer Rust's `match` to `switch`es or nested `if`'s in JS. Because of the `Shape` enum, I think the function reads well.

With these functions defined, all I needed to do was define another closure to use in reducing the `Round`s' score into a total score.

```rust
let calc_score = |total, round: Round| {
    let outcome_score = calc_outcome_score(&round);
    let shape_score = calc_shape_score(&round.my_shape);

    total + outcome_score + shape_score
};
```

The last step was to use `fold` (Rust's reduce) to sum up the total.

```rust
let total = guide.lines().map(create_round).fold(0, calc_score);
```

Part one done.
