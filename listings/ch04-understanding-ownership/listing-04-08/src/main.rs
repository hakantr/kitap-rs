fn ilk_kelime(metin: &String) -> usize {
    let baytlar = metin.as_bytes();

    for (i, &oge) in baytlar.iter().enumerate() {
        if oge == b' ' {
            return i;
        }
    }

    metin.len()
}

// ANCHOR: here
fn main() {
    let mut metin = String::from("merhaba dünya");

    let kelime = ilk_kelime(&metin); // kelime 7 değerini alacak

    metin.clear(); // bu, String'i boşaltır ve onu "" değerine eşitler

    // kelime burada hala 7 değerine sahip, ancak metin'in 7 değeriyle
    // anlamlı bir şekilde kullanabileceğimiz hiçbir içeriği kalmadı,
    // yani kelime artık tamamen geçersiz!
}
// ANCHOR_END: here
