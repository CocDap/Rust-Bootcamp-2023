fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy!");
}
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}
#[test]
fn exercise3_should_work() {
    assert_eq!(Direction::North.opposite(), Direction::South);
    assert_eq!(Direction::East.opposite(), Direction::West);
    assert_eq!(Direction::South.opposite(), Direction::North);
    assert_eq!(Direction::West.opposite(), Direction::East);
}
