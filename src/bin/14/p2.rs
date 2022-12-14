pub fn solve() -> u32 {
    let mut grid = super::parse_input();
    let mut sand_count = 0;

    loop {
        let mut sand = (500, 0);
        sand_count += 1;

        if grid[0][500] == 'o' {
            break;
        }

        loop {
            match find_next(&sand, &grid) {
                Some(next) => sand = next,
                None => {
                    grid[sand.1][sand.0] = 'o';

                    break;
                }
            }
        }
    }

    sand_count - 1
}

fn find_next(sand: &(usize, usize), grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let (x, y) = sand;

    if y + 1 == grid.len() {
        return None;
    }

    if grid[y + 1][*x] == '.' {
        return Some((*x, y + 1));
    }

    if grid[y + 1][x - 1] == '.' {
        return Some((x - 1, y + 1));
    }

    if grid[y + 1][x + 1] == '.' {
        return Some((x + 1, y + 1));
    }

    None
}
