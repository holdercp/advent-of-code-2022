pub fn solve() -> String {
    let (mut supply, procedure) = super::parse_input();

    for step in procedure {
        let stack = &mut supply[step.1 - 1];

        let mut taken: Vec<char> = stack.drain(stack.len() - step.0..).collect();
        taken.reverse();

        supply[step.2 - 1].append(&mut taken);
    }

    let mut top = String::new();
    for mut stack in supply {
        let c = stack.pop().unwrap();
        top.push(c);
    }

    top
}
