use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let karsilama_dosyasi_sonucu = File::open("merhaba.txt");

    let karsilama_dosyasi = match karsilama_dosyasi_sonucu {
        Ok(dosya) => dosya,
        Err(hata) => match hata.kind() {
            ErrorKind::NotFound => match File::create("merhaba.txt") {
                Ok(olusturulan_dosya) => olusturulan_dosya,
                Err(e) => panic!("Dosyayı oluştururken problem oluştu: {e:?}"),
            },
            _ => {
                panic!("Dosyayı açarken problem oluştu: {hata:?}");
            }
        },
    };
}
