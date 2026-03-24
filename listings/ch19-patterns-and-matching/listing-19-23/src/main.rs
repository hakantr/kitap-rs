fn main() {
    // ANCHOR: here
    struct Nokta {
        x: i32,
        y: i32,
        z: i32,
    }

    let koken = Nokta { x: 0, y: 0, z: 0 };

    match koken {
        Nokta { x, .. } => println!("x su: {x}"),
    }
    // ANCHOR_END: here
}
