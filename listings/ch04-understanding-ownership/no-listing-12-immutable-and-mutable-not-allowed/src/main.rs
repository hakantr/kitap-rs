fn main() {
    // ANCHOR: here
    let mut metin = String::from("merhaba");

    let r1 = &metin; // sorun yok
    let r2 = &metin; // sorun yok
    let r3 = &mut metin; // BÜYÜK SORUN

    println!("{r1}, {r2}, ve {r3}");
    // ANCHOR_END: here
}
