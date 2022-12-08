pub fn solve() -> usize {
    let grid = super::parse_input();
    let row_len = grid[0].len();
    let col_len = grid.len();

    let mut visible_trees = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            let height = grid[y][x];

            let is_visible = check_viz((&height, &x, &y), &grid, (&row_len, &col_len));

            if is_visible {
                visible_trees.push(height);
            }
        }
    }

    visible_trees.len()
}

fn check_viz(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>, bounds: (&usize, &usize)) -> bool {
    let (x_bound, y_bound) = bounds;

    let is_visible = check_up(t, g);
    if is_visible {
        return is_visible;
    }
    let is_visible = check_down(t, g, y_bound);
    if is_visible {
        return is_visible;
    }
    let is_visible = check_left(t, g);
    if is_visible {
        return is_visible;
    }
    let is_visible = check_right(t, g, x_bound);
    if is_visible {
        return is_visible;
    }

    is_visible
}

fn check_up(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>) -> bool {
    let (height, x, y) = t;

    for y2 in (0..*y).rev() {
        let other = g[y2][*x];

        if *height <= other {
            return false;
        }
    }

    true
}

fn check_down(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>, bound: &usize) -> bool {
    let (height, x, y) = t;

    for y2 in *y + 1..*bound {
        let other = g[y2][*x];

        if *height <= other {
            return false;
        }
    }

    true
}

fn check_left(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>) -> bool {
    let (height, x, y) = t;

    for x2 in (0..*x).rev() {
        let other = g[*y][x2];

        if *height <= other {
            return false;
        }
    }

    true
}

fn check_right(t: (&u32, &usize, &usize), g: &Vec<Vec<u32>>, bound: &usize) -> bool {
    let (height, x, y) = t;

    for x2 in *x + 1..*bound {
        let other = g[*y][x2];

        if *height <= other {
            return false;
        }
    }

    true
}
