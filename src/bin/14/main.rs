use std::fs;

use regex::Regex;

pub mod p1;
pub mod p2;

fn parse_input() -> Vec<Vec<char>> {
    let input =
        fs::read_to_string("src/bin/14/input.txt").expect("Should have been able to read the file");

    let re_x = Regex::new(r"(\d+),").unwrap();
    let re_y = Regex::new(r",(\d+)").unwrap();

    let x_bound = re_x
        .captures_iter(&input[..])
        .map(|c| c[0][..c[0].len() - 1].parse::<usize>().unwrap())
        .max()
        .unwrap();

    let y_bound = re_y
        .captures_iter(&input[..])
        .map(|c| c[0][1..].parse::<usize>().unwrap())
        .max()
        .unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; x_bound * 2]; y_bound + 2];

    input.lines().for_each(|l| {
        let coors: Vec<(usize, usize)> = l
            .split(" -> ")
            .map(|s| {
                let pair: Vec<usize> = s.split(",").flat_map(|c| c.parse()).collect();
                (pair[0], pair[1])
            })
            .collect();

        for w in coors.windows(2) {
            let first = w[0];
            let second = w[1];

            if w.len() == 2 {
                if first.0 == second.0 {
                    let mn = first.1.min(second.1);
                    let mx = first.1.max(second.1);

                    for y in mn..mx + 1 {
                        grid[y][first.0] = '#';
                    }
                } else if first.1 == second.1 {
                    let mn = first.0.min(second.0);
                    let mx = first.0.max(second.0);

                    for x in mn..mx + 1 {
                        grid[first.1][x] = '#';
                    }
                } else {
                    panic!("bad coordinates: {:?},{:?}", first, second)
                }
            } else {
                grid[first.1][first.0] = '#';
            }
        }
    });

    grid
}

fn main() {
    parse_input();
    println!("Part 1: {}", p1::solve());
    println!("Part 2: {}", p2::solve());
}
