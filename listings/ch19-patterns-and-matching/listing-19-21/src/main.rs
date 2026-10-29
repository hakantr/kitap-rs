fn main() {
    // ANCHOR: here
    let metin = Some(String::from("Merhaba!"));

    if let Some(_metin) = metin {
        println!("bir metin bulundu");
    }

    println!("{metin:?}");
    // ANCHOR_END: here
}
