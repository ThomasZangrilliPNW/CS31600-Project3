enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::West;

    // Exhaustive — all variants covered, compiles fine
    match direction {
        Direction::North => println!("Going north"),
        Direction::South => println!("Going south"),
        Direction::East  => println!("Going east"),
        Direction::West  => println!("Going west"),
    }

    // Non-exhaustive — West is missing, compiler will reject this
    match direction {
        Direction::North => println!("Going north"),
        Direction::South => println!("Going south"),
        Direction::East  => println!("Going east"),
        // West is missing, but _ catches it instead of a compiler error
        _ => println!("Unhandled direction!"),
    }
}