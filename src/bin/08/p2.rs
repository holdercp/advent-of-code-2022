use super::Tree;

pub fn solve() -> usize {
    let grid = super::parse_input();

    grid.iter()
        .flat_map(|r| {
            r.iter().map(|t| {
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

                let score = neighbors.iter().fold(1, |product, n| -> usize {
                    let distance = n.iter().position(|other| other.height >= t.height);

                    let d = match distance {
                        Some(i) => i + 1,
                        None => n.len(),
                    };

                    product * d
                });

                score
            })
        })
        .max()
        .unwrap()
}
