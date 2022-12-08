use super::Tree;

pub fn solve() -> usize {
    let trees = super::parse_input();
    let row_len = &trees.iter().max_by_key(|t| t.y).map(|t| t.y).unwrap();
    let col_len = &trees.iter().max_by_key(|t| t.x).map(|t| t.x).unwrap();

    let mut visible_trees = Vec::new();

    for t in &trees {
        if t.x == 0 || t.x == *col_len || t.y == 0 || t.y == *row_len {
            continue;
        }

        let is_visible = check_viz(&t, &trees);

        if is_visible {
            visible_trees.push(t)
        }
    }

    visible_trees.len() + ((row_len + col_len) * 2)
}

fn check_viz(t: &Tree, trees: &Vec<Tree>) -> bool {
    let up_filter = |other: &&Tree| t.y < other.y && other.x == t.x;
    let down_filter = |other: &&Tree| t.y > other.y && other.x == t.x;
    let left_filter = |other: &&Tree| t.x < other.x && other.y == t.y;
    let right_filter = |other: &&Tree| t.x > other.x && other.y == t.y;

    let get_max_height = |other: &&Tree| other.height;
    let get_height = |other: &Tree| other.height;

    let tallest_up = &trees
        .iter()
        .filter(up_filter)
        .max_by_key(get_max_height)
        .map(get_height)
        .unwrap();

    let tallest_down = &trees
        .iter()
        .filter(down_filter)
        .max_by_key(get_max_height)
        .map(get_height)
        .unwrap();

    let tallest_left = &trees
        .iter()
        .filter(left_filter)
        .max_by_key(get_max_height)
        .map(get_height)
        .unwrap();

    let tallest_right = &trees
        .iter()
        .filter(right_filter)
        .max_by_key(get_max_height)
        .map(get_height)
        .unwrap();

    t.height > *tallest_up
        || t.height > *tallest_down
        || t.height > *tallest_left
        || t.height > *tallest_right
}
