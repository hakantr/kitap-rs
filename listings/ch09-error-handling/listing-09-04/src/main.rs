use std::fs::File;

fn main() {
    let karsilama_dosyasi_sonucu = File::open("merhaba.txt");

    let karsilama_dosyasi = match karsilama_dosyasi_sonucu {
        Ok(dosya) => dosya,
        Err(hata) => panic!("Dosyayı açarken problem oluştu: {hata:?}"),
    };
}
