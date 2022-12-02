fn get_shape_score(shape: &str) -> u32 {
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("bad shape"),
    }
}

fn get_outcome_score(p1: &str, p2: &str) -> u32 {
    match p1 {
        "A" => match p2 {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => panic!("bad shape"),
        },
        "B" => match p2 {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("bad shape"),
        },
        "C" => match p2 {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => panic!("bad shape"),
        },
        _ => panic!("bad shape"),
    }
}

pub fn solve() -> u32 {
    let guide = super::read_input();

    let mut score: u32 = 0;

    for l in guide.lines() {
        let shapes: Vec<&str> = l.split_whitespace().collect();
        let outcome_score = get_outcome_score(shapes[0], shapes[1]);
        let shape_score = get_shape_score(shapes[1]);

        score += outcome_score + shape_score;
    }

    score
}
