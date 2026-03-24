fn main() {
    // ANCHOR: here
    let x = 'c';

    match x {
        'a'..='j' => println!("erken ASCII harfi"),
        'k'..='z' => println!("geç ASCII harfi"),
        _ => println!("başka bir şey"),
    }
    // ANCHOR_END: here
}
