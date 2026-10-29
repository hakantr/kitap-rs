fn main() {
    // ANCHOR: here
    let metin = Some(String::from("Merhaba!"));

    if let Some(_) = metin {
        println!("bir metin bulundu");
    }

    println!("{metin:?}");
    // ANCHOR_END: here
}
