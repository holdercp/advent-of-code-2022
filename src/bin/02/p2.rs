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

fn choose_shape<'a>(p1: &'a str, outcome: &'a str) -> &'a str {
    match p1 {
        "A" => match outcome {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => panic!("bad shape"),
        },
        "B" => match outcome {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => panic!("bad shape"),
        },
        "C" => match outcome {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => panic!("bad shape"),
        },
        _ => panic!("bad shape"),
    }
}

pub fn solve() -> u32 {
    let guide = super::read_input();

    let mut score: u32 = 0;

    for l in guide.lines() {
        let res: Vec<&str> = l.split_whitespace().collect();
        let shape = choose_shape(res[0], res[1]);
        let outcome_score = get_outcome_score(res[0], shape);
        let shape_score = get_shape_score(shape);

        score += outcome_score + shape_score;
    }

    score
}
