enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Outcome {
    Lose(i32),
    Draw(i32),
    Win(i32),
}

struct Round {
    opponent_shape: Shape,
    outcome: Outcome,
}

fn calc_round_score(round: &Round) -> i32 {
    match round.opponent_shape {
        Shape::Rock => match round.outcome {
            Outcome::Lose(score) => score + Shape::Scissors.score(),
            Outcome::Draw(score) => score + Shape::Rock.score(),
            Outcome::Win(score) => score + Shape::Paper.score(),
        },
        Shape::Paper => match round.outcome {
            Outcome::Lose(score) => score + Shape::Rock.score(),
            Outcome::Draw(score) => score + Shape::Paper.score(),
            Outcome::Win(score) => score + Shape::Scissors.score(),
        },
        Shape::Scissors => match round.outcome {
            Outcome::Lose(score) => score + Shape::Paper.score(),
            Outcome::Draw(score) => score + Shape::Scissors.score(),
            Outcome::Win(score) => score + Shape::Rock.score(),
        },
    }
}

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

pub fn solve() -> i32 {
    let guide = super::read_input();

    let create_round = |line: &str| {
        let cols: Vec<&str> = line.split_whitespace().collect();
        Round {
            opponent_shape: create_shape(cols[0]),
            outcome: create_outcome(cols[1]),
        }
    };

    let calc_score = |total, round: Round| total + calc_round_score(&round);

    let total = guide.lines().map(create_round).fold(0, calc_score);

    total
}
