// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn dosyadan_kullanici_adini_oku() -> Result<String, io::Error> {
    let mut kullanici_adi = String::new();

    File::open("merhaba.txt")?.read_to_string(&mut kullanici_adi)?;

    Ok(kullanici_adi)
}
// ANCHOR_END: here

fn main() {
    let kullanici_adi =
        dosyadan_kullanici_adini_oku().expect("Kullanıcı adı alınamadı");
}
