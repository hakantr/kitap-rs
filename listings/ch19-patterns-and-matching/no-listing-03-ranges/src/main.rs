fn main() {
    // ANCHOR: here
    let x = 5;

    match x {
        1..=5 => println!("birden beşe kadar"),
        _ => println!("başka bir şey"),
    }
    // ANCHOR_END: here
}
