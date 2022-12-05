pub fn solve() -> String {
    let (mut supply, procedure) = super::parse_input();

    for instruction in procedure {
        let mut temp: Vec<char> = Vec::new();
        for _time in 0..instruction.0 {
            let c = supply[instruction.1 - 1].pop().unwrap();
            temp.push(c);
        }
        supply[instruction.2 - 1].extend(temp.iter().rev());
    }

    let mut top = String::new();
    for mut krates in supply {
        let c = krates.pop().unwrap();
        top.push(c);
    }

    top
}
