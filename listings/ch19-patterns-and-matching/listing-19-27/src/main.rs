fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50 alındı"),
        Some(n) if n == y => println!("Eşleşti, n = {n}"),
        _ => println!("Varsayılan durum, x = {x:?}"),
    }

    println!("sonda: x = {x:?}, y = {y}");
}
