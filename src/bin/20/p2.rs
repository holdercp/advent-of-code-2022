pub fn solve() -> i64 {
    let file = super::parse_input();
    let file: Vec<(i64, usize)> = file
        .iter()
        .map(|(n, i)| (*n as i64 * 811589153, *i))
        .collect();

    let mut mixed = file.clone();

    for _ in 0..10 {
        for num in &file {
            let pos = mixed.iter().position(|n| n == num).unwrap();
            let wrapped_pos = wrap(pos as i64 + num.0, mixed.len() - 1);

            let n = mixed.remove(pos);

            mixed.insert(wrapped_pos, n);
        }
    }

    let pos0 = mixed.iter().position(|n| n.0 == 0).unwrap();

    mixed[(pos0 + 1_000) % mixed.len()].0
        + mixed[(pos0 + 2_000) % mixed.len()].0
        + mixed[(pos0 + 3_000) % mixed.len()].0
}

fn wrap(n: i64, len: usize) -> usize {
    let offset_count = if n.is_negative() {
        (-n / len as i64) + 1
    } else {
        0
    };

    ((n + len as i64 * offset_count) % len as i64)
        .try_into()
        .unwrap()
}
