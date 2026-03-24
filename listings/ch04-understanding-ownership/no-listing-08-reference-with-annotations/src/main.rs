fn main() {
    let metin1 = String::from("merhaba");

    let uzunluk = uzunlugu_hesapla(&metin1);

    println!("'{metin1}' metninin uzunluğu: {uzunluk}.");
}

// ANCHOR: here
fn uzunlugu_hesapla(metin: &String) -> usize {
    // metin bir String'e referanstır
    metin.len()
} // Burada, metin kapsam dışına çıkar. Ancak metin referans verdiği şeyin
  // sahipliğine sahip olmadığı için String düşürülmez (drop edilmez).
  // ANCHOR_END: here
