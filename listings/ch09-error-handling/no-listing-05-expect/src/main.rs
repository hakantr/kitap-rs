use std::fs::File;

fn main() {
    let karsilama_dosyasi = File::open("merhaba.txt")
        .expect("merhaba.txt bu projeye dahil edilmelidir");
}
