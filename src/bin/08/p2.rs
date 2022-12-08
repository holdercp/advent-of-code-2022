pub fn solve() -> u32 {
    let grid = super::parse_input_2();

    let row_len = grid[0].len();
    let col_len = grid.len();

    let mut scores: Vec<u32> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            let height = grid[y][x];
            scores.push(compute_scenic_score(
                (&height, &x, &y),
                &grid,
                (&row_len, &col_len),
            ))
        }
    }

    *scores.iter().max().unwrap()
}

fn compute_scenic_score(
    t: (&u32, &usize, &usize),
    g: &Vec<Vec<u32>>,
    bounds: (&usize, &usize),
) -> u32 {
    let (x_bound, y_bound) = bounds;

    let up_distance = get_up_distance(t, &g);
    let down_distance = get_down_distance(t, &g, y_bound);
    let left_distance = get_left_distance(t, &g);
    let right_distance = get_right_distance(t, &g, x_bound);

    up_distance * down_distance * left_distance * right_distance
}

fn get_up_distance(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>) -> u32 {
    let (height, x, y) = t;

    let mut distance = 0;

    for y2 in (0..*y).rev() {
        let other = g[y2][*x];

        distance += 1;

        if *height <= other {
            break;
        }
    }

    distance
}

fn get_down_distance(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>, bound: &usize) -> u32 {
    let (height, x, y) = t;

    let mut distance = 0;

    for y2 in *y + 1..*bound {
        let other = g[y2][*x];

        distance += 1;

        if *height <= other {
            break;
        }
    }

    distance
}

fn get_left_distance(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>) -> u32 {
    let (height, x, y) = t;

    let mut distance = 0;

    for x2 in (0..*x).rev() {
        let other = g[*y][x2];

        distance += 1;

        if *height <= other {
            break;
        }
    }

    distance
}

fn get_right_distance(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>, bound: &usize) -> u32 {
    let (height, x, y) = t;

    let mut distance = 0;

    for x2 in *x + 1..*bound {
        let other = g[*y][x2];

        distance += 1;

        if *height <= other {
            break;
        }
    }

    distance
}
