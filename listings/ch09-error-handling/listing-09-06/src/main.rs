// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn dosyadan_kullanici_adini_oku() -> Result<String, io::Error> {
    let kullanici_adi_dosyasi_sonucu = File::open("merhaba.txt");

    let mut kullanici_adi_dosyasi = match kullanici_adi_dosyasi_sonucu {
        Ok(dosya) => dosya,
        Err(e) => return Err(e),
    };

    let mut kullanici_adi = String::new();

    match kullanici_adi_dosyasi.read_to_string(&mut kullanici_adi) {
        Ok(_) => Ok(kullanici_adi),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let kullanici_adi =
        dosyadan_kullanici_adini_oku().expect("Kullanıcı adı alınamadı");
}
