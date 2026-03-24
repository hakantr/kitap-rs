fn main() {
    // ANCHOR: here
    let mut metin = String::from("merhaba");

    {
        let r1 = &mut metin;
    } // r1 burada kapsam dışına çıkar, bu yüzden hiçbir sorun olmadan yeni bir referans yapabiliriz.

    let r2 = &mut metin;
    // ANCHOR_END: here
}
