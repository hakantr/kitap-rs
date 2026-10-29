struct Nokta {
    x: i32,
    y: i32,
}

// ANCHOR: here
fn main() {
    let nokta = Nokta { x: 0, y: 7 };

    match nokta {
        Nokta { x, y: 0 } => println!("x ekseni üzerinde: {x}"),
        Nokta { x: 0, y } => println!("y ekseni üzerinde: {y}"),
        Nokta { x, y } => {
            println!("Hiçbir eksen üzerinde değil: ({x}, {y})");
        }
    }
}
// ANCHOR_END: here
