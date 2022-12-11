const ROUNDS: usize = 20;

pub fn solve() -> u32 {
    let mut monkeys = super::parse_input();

    let lcm: u64 = 9_699_690;

    for _ in 0..ROUNDS {
        for id in 0..monkeys.len() {
            let mut monkey = monkeys.remove(&id).unwrap();

            while !monkey.items.is_empty() {
                let mut item = monkey.inspect_item();

                let new_worry = monkey.operation.execute(&item);

                let new_worry = new_worry / 3;

                item.worry = new_worry;

                let other_id = monkey.test(&item);

                monkeys.get_mut(&other_id).unwrap().items.push_back(item);
            }

            monkeys.insert(id, monkey);
        }
    }

    let mut inspected = monkeys.values().map(|m| m.inspected).collect::<Vec<u32>>();
    inspected.sort();

    let largest = &inspected[inspected.len() - 2..];

    largest.iter().product()
}
