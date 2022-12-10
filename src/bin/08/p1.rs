use super::Tree;

pub fn solve() -> usize {
    let grid = super::parse_input();

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

    grid.iter()
        .flat_map(|r| r.iter().filter(visible_trees_filter))
        .count()
}
