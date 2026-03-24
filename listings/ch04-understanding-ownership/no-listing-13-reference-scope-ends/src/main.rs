fn main() {
    // ANCHOR: here
    let mut metin = String::from("merhaba");

    let r1 = &metin; // sorun yok
    let r2 = &metin; // sorun yok
    println!("{r1} ve {r2}");
    // r1 ve r2 değişkenleri bu noktadan sonra kullanılmayacak.

    let r3 = &mut metin; // sorun yok
    println!("{r3}");
    // ANCHOR_END: here
}
