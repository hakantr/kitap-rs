struct Nokta {
    x: i32,
    y: i32,
}

// ANCHOR: here
fn main() {
    let nokta = Nokta { x: 0, y: 7 };

    match nokta {
        Nokta { x, y: 0 } => println!("x ekseni uzerinde: {x}"),
        Nokta { x: 0, y } => println!("y ekseni uzerinde: {y}"),
        Nokta { x, y } => {
            println!("Hicbir eksen uzerinde degil: ({x}, {y})");
        }
    }
}
// ANCHOR_END: here
