pub fn solve() -> String {
    let (mut supply, procedure) = super::parse_input();

    for step in procedure {
        let stack = &mut supply[step.1 - 1];

        let mut taken: Vec<char> = stack.crates.drain(stack.crates.len() - step.0..).collect();
        taken.reverse();

        supply[step.2 - 1].crates.append(&mut taken);
    }

    let mut top = String::new();
    for mut stack in supply {
        let c = stack.crates.pop().unwrap();
        top.push(c);
    }

    top
}
