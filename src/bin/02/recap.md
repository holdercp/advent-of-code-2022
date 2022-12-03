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

## Part Two

Well, in typical Advent of Code fashion, we have our first twist. The second column of the input doesn't represent our _shape_, it represents the outcome we want to get for that round. I had to sit and think about this change for a bit but since I spent some time strucutring my data well in part one, I didn't have to make many adjustments for part two.

The first change I made was to create an `Outcome` enum. I thought it made a lot of sense for each outcome to hold its score as well. This would let us remove our `calc_outcome_score` function from part 1.

```rust
enum Outcome {
    Lose(i32),
    Draw(i32),
    Win(i32),
}
```

I also updated my `Round` struct to represent the change.

```rust
struct Round {
    opponent_shape: Shape,
    outcome: Outcome,
}
```

Now that there's two types represented in a `Round` instead of just `Shape`, I needed to update my `create_shape` builder and create a new `create_outcome` builder so I can construct the new `Round`s.

```rust
fn create_shape(shape_raw: &str) -> Shape {
    match shape_raw {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("bad shape"),
    }
}

fn create_outcome(outcome_raw: &str) -> Outcome {
    match outcome_raw {
        "X" => Outcome::Lose(0),
        "Y" => Outcome::Draw(3),
        "Z" => Outcome::Win(6),
        _ => panic!("bad outcome"),
    }
}

let create_round = |line: &str| {
    let cols: Vec<&str> = line.split_whitespace().collect();
    Round {
        opponent_shape: create_shape(cols[0]),
        outcome: create_outcome(cols[1]),
    }
};
```

Now that I had my new `Round` structure correct, all I needed to do was calculate the score for each one.

Since the scoring didn't change from part 1, and I added the score directly on the `Outcome` enum, I could just `match` on the `Round.opponent_shape`, `match` on the `Round.outcome`, and then return the `Outcome` score with the `Shape` score appropriate for the outcome. In otherwords, since I know the outcome and what the opponent played, I know what `Shape` I would need to play (and the associated score) to get that outcome.

My first implementation looked something like this:

```rust
fn calc_round_score(round: &Round) -> i32 {
    match round.opponent_shape {
        Shape::Rock => match round.outcome {
            Outcome::Lose(score) => score + calc_shape_score(&Shape::Scissors),
            Outcome::Draw(score) => score + calc_shape_score(&Shape::Rock),
            Outcome::Win(score) => score + calc_shape_score(&Shape::Paper),
        },
        Shape::Paper => match round.outcome {
          //  ...
        },
        Shape::Scissors => match round.outcome {
          //  ...
        },
    }
}
```

Then I had the thought that I should just add a method to the `Shape` to get its own score. That made things a bit cleaner.

```rust
impl Shape {
    fn get_score(&self) -> i32 {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn calc_round_score(round: &Round) -> i32 {
    match round.opponent_shape {
        Shape::Rock => match round.outcome {
            Outcome::Lose(score) => score + &Shape::Scissors.get_score(),
            Outcome::Draw(score) => score + &Shape::Rock.get_score(),
            Outcome::Win(score) => score + &Shape::Paper.get_score(),
        },
        Shape::Paper => match round.outcome {
            Outcome::Lose(score) => score + &Shape::Rock.get_score(),
            Outcome::Draw(score) => score + &Shape::Paper.get_score(),
            Outcome::Win(score) => score + &Shape::Scissors.get_score(),
        },
        Shape::Scissors => match round.outcome {
            Outcome::Lose(score) => score + &Shape::Paper.get_score(),
            Outcome::Draw(score) => score + &Shape::Scissors.get_score(),
            Outcome::Win(score) => score + &Shape::Rock.get_score(),
        },
    }
}
```

With all this in place, the last step was to update my `fold` closure to sum up the total.

```rust
let calc_score = |total, round: Round| total + calc_round_score(&round);

let total = guide.lines().map(create_round).fold(0, calc_score);
```

Part two done.
