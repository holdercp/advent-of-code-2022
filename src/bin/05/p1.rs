pub fn solve() -> String {
    let (mut supply, procedure) = super::parse_input();

    for instruction in procedure {
        for _time in 0..instruction.0 {
            println!("{:?}", instruction);
            let c = supply[instruction.1 - 1].pop().unwrap();
            supply[instruction.2 - 1].push(c);
        }
    }

    let mut top = String::new();
    for mut krates in supply {
        let c = krates.pop().unwrap();
        top.push(c);
    }

    top
}
