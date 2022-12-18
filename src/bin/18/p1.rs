use std::collections::HashSet;

pub fn solve() -> u32 {
    let input = super::read_input();

    let mut cubes: HashSet<Cube> = HashSet::new();

    let mut area = 0;

    input
        .lines()
        .map(|l| {
            let coors: Vec<u32> = l.split(',').map(|n| n.parse().unwrap()).collect();

            Cube::new(coors[0], coors[1], coors[2])
        })
        .for_each(|c| {
            let adj_cubes = c.build_adj_cubes();

            let open_sides: u32 = adj_cubes.difference(&cubes).count().try_into().unwrap();
            let closed_sides = c.sides - open_sides;

            area += open_sides - closed_sides;

            cubes.insert(c);
        });

    area
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
    sides: u32,
}

impl Cube {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z, sides: 6 }
    }

    fn build_adj_cubes(&self) -> HashSet<Cube> {
        HashSet::from([
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y, self.z - 1),
            Cube::new(self.x, self.y, self.z + 1),
        ])
    }
}
