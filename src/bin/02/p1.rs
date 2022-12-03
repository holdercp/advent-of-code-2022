enum Shape {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    opponent_shape: Shape,
    my_shape: Shape,
}

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

pub fn solve() -> i32 {
    let guide = super::read_input();

    let create_round = |line: &str| {
        let cols: Vec<&str> = line.split_whitespace().collect();
        Round {
            opponent_shape: create_shape(cols[0]),
            my_shape: create_shape(cols[1]),
        }
    };

    let calc_score = |total, round: Round| {
        let outcome_score = calc_outcome_score(&round);
        let shape_score = calc_shape_score(&round.my_shape);

        total + outcome_score + shape_score
    };

    let total = guide.lines().map(create_round).fold(0, calc_score);

    total
}
