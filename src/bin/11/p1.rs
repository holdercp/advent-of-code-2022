const ROUNDS: usize = 20;

pub fn solve() -> u64 {
    let mut monkeys = super::parse_input();

    for _ in 0..ROUNDS {
        for id in 0..monkeys.len() {
            let mut monkey = monkeys.remove(&id).unwrap();

            while !monkey.items.is_empty() {
                let mut item = monkey.inspect_item();

                item.worry = monkey.operation.execute(&item) / 3;

                let other_id = monkey.test(&item);

                monkeys.get_mut(&other_id).unwrap().items.push_back(item);
            }

            monkeys.insert(id, monkey);
        }
    }

    let mut inspected = monkeys.values().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort();

    let largest = &inspected[inspected.len() - 2..];

    largest.iter().product()
}
