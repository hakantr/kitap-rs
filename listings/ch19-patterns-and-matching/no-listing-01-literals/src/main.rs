fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 => println!("bir"),
        2 => println!("iki"),
        3 => println!("üç"),
        _ => println!("herhangi bir şey"),
    }
    // ANCHOR_END: here
}
