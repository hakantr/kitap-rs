// ANCHOR: here
use std::fs;
use std::io;

fn dosyadan_kullanici_adini_oku() -> Result<String, io::Error> {
    fs::read_to_string("merhaba.txt")
}
// ANCHOR_END: here

fn main() {
    let kullanici_adi =
        dosyadan_kullanici_adini_oku().expect("Kullanıcı adı alınamadı");
}
