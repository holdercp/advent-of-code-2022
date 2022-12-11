# Rope Bridge

## Part One

This puzzle was an interesting one. It took me awhile to get the movements down, but I dabbled with a little more OOP in Rust with this one. I will say, Rust is not built to be tradionally OOP, and I found it very difficult to break out of an inheritance way of thinking.

Most of my code lives in the `main.rs` file so it can be shared between the two parts (I know there's a better way to strucutre shared code but this is easy for what I need to do in Advent). The unique code for the different parts are essentially just the loop to execute the motions; the structs take care of the rest.

First, the models.

I created an enum for the motions the head can take. These enums contained their own data as well so it was really nice to be able to match on the type of motion and move based on the value. Because of this, I parsed the input to be a vector of motions.

```rust
fn parse_input() -> Vec<Motion> {
    let input =
        fs::read_to_string("src/bin/09/input.txt").expect("Should have been able to read the file");

    let motions = input
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();

            let dir: char = split[0].chars().next().unwrap();
            let steps: i32 = split[1].parse().unwrap();

            match dir {
                'U' => Motion::U(steps),
                'D' => Motion::D(steps),
                'L' => Motion::L(steps),
                'R' => Motion::R(steps),
                _ => panic!("bad direction"),
            }
        })
        .collect();

    motions
}

enum Motion {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

```

Next I created a `Point` tuple struct. This would serve to be the common type for representing a knot's postion on the grid.

```rust
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point(i32, i32);
```

Next up the `Knot` trait. This is where I was really trying to force Rust to do inheritance and I failed. I think the solutions is ok but I would love a Rust expert to help me express the model I was going for. Anyway, the `Knot` trait requires a few functions that I need on both the `Head` and the `Tail`. This let me use "bounded" traits so if I needed to work with a Head or a Tail they both implemented `Knot` so the compiler knew they had the specific methods I needed.

```rust
trait Knot {
    fn new() -> Self;

    fn get_position(&self) -> Point;
    fn set_position(&mut self, point: Point);
}
```

I created a `Head` struct and implemented a `move` method that read a `Motion` and update the Head's position accordingly.

```rust
struct Head {
    position: Point,
}

impl Head {
    fn r#move(&mut self, motion: &Motion) {
        let Point(x, y) = self.get_position();

        match motion {
            Motion::U(steps) => self.set_position(Point(x, y + steps)),
            Motion::D(steps) => self.set_position(Point(x, y - steps)),
            Motion::L(steps) => self.set_position(Point(x - steps, y)),
            Motion::R(steps) => self.set_position(Point(x + steps, y)),
        };
    }
}

impl Knot for Head {
    fn new() -> Self {
        Self {
            position: Point(0, 0),
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
}
```

I added a `Tail` struct as well that had a `position` but also had a `visited` hash set to keep track of it's position history and filter out duplicates automatically.

```rust
struct Tail {
    position: Point,
    visited: HashSet<Point>,
}

impl Tail {
    fn update_visited(&mut self) {
        self.visited.insert(self.get_position());
    }
}

impl Knot for Tail {
    fn new() -> Self {
        Self {
            position: Point(0, 0),
            visited: HashSet::new(),
        }
    }
    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, point: Point) {
        self.position = point;
    }
}
```

Most of the movement logic is defined/required in/by a `Follower` trait. The Tail implements this trait because instead of moving directly to its new position like Head does, it "steps" towards its leader. I also added the method to check whether or not the Follower should move at all. If it's adjacent to its leader, it doesn't need to move.

```rust
trait Follower
where
    Self: Knot,
{
    fn is_adjacent(&self, leader: &impl Knot) -> bool {
        let Point(x, y) = self.get_position();
        let Point(lx, ly) = leader.get_position();

        let x_dis = x.abs_diff(lx);
        let y_dis = y.abs_diff(ly);

        x_dis <= 1 && y_dis <= 1
    }

    fn step(&mut self, leader: &impl Knot) {
        if self.is_adjacent(leader) {
            return;
        }

        let Point(x, y) = self.get_position();
        let Point(lx, ly) = leader.get_position();

        let x_dis = x.abs_diff(lx);
        let y_dis = y.abs_diff(ly);

        let x_dir = if lx - x < 0 { -1 } else { 1 };
        let y_dir = if ly - y < 0 { -1 } else { 1 };

        if x_dis == 0 {
            self.set_position(Point(x, y + y_dir));
        } else if y_dis == 0 {
            self.set_position(Point(x + x_dir, y));
        } else {
            self.set_position(Point(x + x_dir, y + y_dir));
        }
    }
}
```

The `Self: Knot` bit was key. It lets me state that the struct that implements Follower must also implement Knot. This tied together the movement behavior and the position behavior.

The logic for the `step` method was pretty straight forward once I looked at enough examples in the prompt. The steps look like this:

1. Check if the Follower is adjacent to its leader. If so, it early returns and is a noop. Otherwise, continue.
2. Get the position of the Follower and its leader.
3. Calculate the absolute x distance and y distance between them.
4. Calculate the x direction and y direction the Follower needs to move to get closer to its leader.
5. If the x distance is `0` (meaning the Follower is above or below its leader), then increment the y coordinate in the y direction.
6. Do the opposite if the y distance is `0` (the Follower is to the left or right of the leader).
7. Otherwise, move diagonally towards the leader by incrmenting both x and y in their respective directions.

The `Tail` struct actually defined it's own `step` because it also needed to call its `update_visited` method after every step.

```rust
impl Follower for Tail {
    fn step(&mut self, leader: &impl Knot) {
        if self.is_adjacent(leader) {
            return;
        }

        let Point(x, y) = self.get_position();
        let Point(lx, ly) = leader.get_position();

        let x_dis = x.abs_diff(lx);
        let y_dis = y.abs_diff(ly);

        let x_dir = if lx - x < 0 { -1 } else { 1 };
        let y_dir = if ly - y < 0 { -1 } else { 1 };

        if x_dis == 0 {
            self.set_position(Point(x, y + y_dir));
        } else if y_dis == 0 {
            self.set_position(Point(x + x_dir, y));
        } else {
            self.set_position(Point(x + x_dir, y + y_dir));
        }

        self.update_visited();
    }
}
```

Now the main loop.

I initialized a `Head` and `Tail` struct and called the Tail's `update_visited` method to log the `0,0` position. Then I iterated through each motion from the input. The `Head` would move to it's new position and the `Tail` would take steps defined by the motion towards the Head.

After all the motions, I returned the length of the Tail's visited set.

Done.

## Part Two

Part two just required adding a new Knot type that I called `Segment`. It implemented `Knot` and `Follower` but didn't have a hash set to keep track of its positions. Other that that it is the same as the Tail.

I instantiated eight of them and each of them took their steps after the Head had moved.

```rust
pub fn solve() -> usize {
    let motions = super::parse_input();
    let mut head = Head::new();
    let mut segment1 = Segment::new();
    let mut segment2 = Segment::new();
    let mut segment3 = Segment::new();
    let mut segment4 = Segment::new();
    let mut segment5 = Segment::new();
    let mut segment6 = Segment::new();
    let mut segment7 = Segment::new();
    let mut segment8 = Segment::new();
    let mut tail = Tail::new();

    tail.update_visited();

    motions.iter().for_each(|m| {
        head.r#move(m);

        let steps = match m {
            Motion::U(steps) => steps,
            Motion::D(steps) => steps,
            Motion::L(steps) => steps,
            Motion::R(steps) => steps,
        };

        for _ in 0..*steps {
            segment1.step(&head);
            segment2.step(&segment1);
            segment3.step(&segment2);
            segment4.step(&segment3);
            segment5.step(&segment4);
            segment6.step(&segment5);
            segment7.step(&segment6);
            segment8.step(&segment7);
            tail.step(&segment8);
        }
    });

    tail.visited.len()
}
```

Part two done.
