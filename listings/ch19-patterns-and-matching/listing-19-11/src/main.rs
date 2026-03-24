fn main() {
    // ANCHOR: here
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50 alındı"),
        Some(y) => println!("Eşleşti, y = {y}"),
        _ => println!("Varsayılan durum, x = {x:?}"),
    }

    println!("sonda: x = {x:?}, y = {y}");
    // ANCHOR_END: here
}
