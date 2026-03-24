fn main() {
    let metin1 = String::from("merhaba");

    let (metin2, uzunluk) = uzunlugu_hesapla(metin1);

    println!("'{metin2}' metninin uzunluğu: {uzunluk}.");
}

fn uzunlugu_hesapla(metin: String) -> (String, usize) {
    let uzunluk = metin.len(); // len() bir String'in uzunluğunu döndürür

    (metin, uzunluk)
}
