// ANCHOR: all
fn main() {
    // ANCHOR: here
    let metin1 = String::from("merhaba");

    let uzunluk = uzunlugu_hesapla(&metin1);
    // ANCHOR_END: here

    println!("'{metin1}' metninin uzunluğu: {uzunluk}.");
}

fn uzunlugu_hesapla(metin: &String) -> usize {
    metin.len()
}
// ANCHOR_END: all
