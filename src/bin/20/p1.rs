pub fn solve() -> i32 {
    let file = super::parse_input();

    let mut mixed = file.clone();

    for num in &file {
        let pos = mixed.iter().position(|n| n == num).unwrap();
        let wrapped_pos = wrap(pos as i32 + num.0, mixed.len() - 1);

        let n = mixed.remove(pos);

        mixed.insert(wrapped_pos, n);
    }

    let pos0 = mixed.iter().position(|n| n.0 == 0).unwrap();

    mixed[(pos0 + 1_000) % mixed.len()].0
        + mixed[(pos0 + 2_000) % mixed.len()].0
        + mixed[(pos0 + 3_000) % mixed.len()].0
}

fn wrap(n: i32, len: usize) -> usize {
    let offset_count = if n.is_negative() {
        (-n / len as i32) + 1
    } else {
        0
    };

    ((n + len as i32 * offset_count) % len as i32)
        .try_into()
        .unwrap()
}
