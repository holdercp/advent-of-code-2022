use std::collections::HashMap;

use crate::{Monkey, Operation};

pub fn solve() -> f64 {
    let monkeys = super::parse_input();

    let root = monkeys.get("root").unwrap();

    let left = monkeys.get(&root.others[0]).unwrap();
    let right = monkeys.get(&root.others[1]).unwrap();

    let left_res = compute_monkey(left, &monkeys, &Vec::new());
    let right_res = compute_monkey(right, &monkeys, &Vec::new());

    let process_ops = |acc: f64, op: &InverseOperation| match op {
        InverseOperation::Add(operand) => acc + operand,
        InverseOperation::Subtract(operand) => acc - operand,
        InverseOperation::SubtractBy(operand) => (acc - operand) * -1.0,
        InverseOperation::Multiply(operand) => acc * operand,
        InverseOperation::Divide(operand) => acc / operand,
        InverseOperation::DivideBy(operand) => operand / acc,
    };

    if left_res.1.is_empty() {
        if let Resolution::Number(res) = left_res.0 {
            right_res.1.iter().fold(res, process_ops)
        } else {
            panic!("should not get here")
        }
    } else if let Resolution::Number(res) = right_res.0 {
        left_res.1.iter().rfold(res, process_ops)
    } else {
        1.0
    }
}

fn compute_monkey(
    m: &Monkey,
    monkeys: &HashMap<String, Monkey>,
    inverse_ops: &Vec<InverseOperation>,
) -> (Resolution, Vec<InverseOperation>) {
    if m.name == "humn" {
        return (Resolution::Me, inverse_ops.to_vec());
    }

    if m.others.is_empty() {
        return (Resolution::Number(m.number), inverse_ops.to_vec());
    }

    let other1 = monkeys.get(&m.others[0]).unwrap();
    let other2 = monkeys.get(&m.others[1]).unwrap();

    match m.operation {
        Operation::Add => {
            let res = compute_monkey(other1, monkeys, inverse_ops);
            let res2 = compute_monkey(other2, monkeys, inverse_ops);

            match res.0 {
                Resolution::Number(operand) => match res2.0 {
                    Resolution::Number(operand2) => {
                        (Resolution::Number(operand + operand2), inverse_ops.to_vec())
                    }
                    Resolution::Me => {
                        let mut ops_copy = res2.1.to_vec();
                        ops_copy.push(InverseOperation::Subtract(operand));

                        (res2.0, ops_copy)
                    }
                },
                Resolution::Me => {
                    if let Resolution::Number(operand) = res2.0 {
                        let mut ops_copy = res.1.to_vec();
                        ops_copy.push(InverseOperation::Subtract(operand));

                        (res.0, ops_copy)
                    } else {
                        panic!("should not happen");
                    }
                }
            }
        }

        Operation::Subtract => {
            let res = compute_monkey(other1, monkeys, inverse_ops);
            let res2 = compute_monkey(other2, monkeys, inverse_ops);

            match res.0 {
                Resolution::Number(operand) => match res2.0 {
                    Resolution::Number(operand2) => {
                        (Resolution::Number(operand - operand2), inverse_ops.to_vec())
                    }
                    Resolution::Me => {
                        let mut ops_copy = res2.1.to_vec();
                        ops_copy.push(InverseOperation::SubtractBy(operand));

                        (res2.0, ops_copy)
                    }
                },
                Resolution::Me => {
                    if let Resolution::Number(operand) = res2.0 {
                        let mut ops_copy = res.1.to_vec();
                        ops_copy.push(InverseOperation::Add(operand));

                        (res.0, ops_copy)
                    } else {
                        panic!("should not happen");
                    }
                }
            }
        }

        Operation::Multiply => {
            let res = compute_monkey(other1, monkeys, inverse_ops);
            let res2 = compute_monkey(other2, monkeys, inverse_ops);

            match res.0 {
                Resolution::Number(operand) => match res2.0 {
                    Resolution::Number(operand2) => {
                        (Resolution::Number(operand * operand2), inverse_ops.to_vec())
                    }
                    Resolution::Me => {
                        let mut ops_copy = res2.1.to_vec();
                        ops_copy.push(InverseOperation::Divide(operand));

                        (res2.0, ops_copy)
                    }
                },
                Resolution::Me => {
                    if let Resolution::Number(operand) = res2.0 {
                        let mut ops_copy = res.1.to_vec();
                        ops_copy.push(InverseOperation::Divide(operand));

                        (res.0, ops_copy)
                    } else {
                        panic!("should not happen");
                    }
                }
            }
        }

        Operation::Divide => {
            let res = compute_monkey(other1, monkeys, inverse_ops);
            let res2 = compute_monkey(other2, monkeys, inverse_ops);

            match res.0 {
                Resolution::Number(operand) => match res2.0 {
                    Resolution::Number(operand2) => {
                        (Resolution::Number(operand / operand2), inverse_ops.to_vec())
                    }
                    Resolution::Me => {
                        let mut ops_copy = res2.1.to_vec();
                        ops_copy.push(InverseOperation::DivideBy(operand));

                        (res2.0, ops_copy)
                    }
                },
                Resolution::Me => {
                    if let Resolution::Number(operand) = res2.0 {
                        let mut ops_copy = res.1.to_vec();
                        ops_copy.push(InverseOperation::Multiply(operand));

                        (res.0, ops_copy)
                    } else {
                        panic!("should not happen");
                    }
                }
            }
        }
        Operation::None => panic!("this shouldn't have happened"),
    }
}

enum Resolution {
    Me,
    Number(f64),
}

#[derive(Clone)]
enum InverseOperation {
    Add(f64),
    Subtract(f64),
    SubtractBy(f64),
    Multiply(f64),
    Divide(f64),
    DivideBy(f64),
}
