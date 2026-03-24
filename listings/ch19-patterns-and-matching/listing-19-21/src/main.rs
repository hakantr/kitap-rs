fn main() {
    // ANCHOR: here
    let dizgi = Some(String::from("Merhaba!"));

    if let Some(_dizgi) = dizgi {
        println!("bir dizgi bulundu");
    }

    println!("{dizgi:?}");
    // ANCHOR_END: here
}
