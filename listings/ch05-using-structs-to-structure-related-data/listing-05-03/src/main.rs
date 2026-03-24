struct Kullanici {
    aktif: bool,
    kullanici_adi: String,
    eposta: String,
    giris_sayisi: u64,
}

// ANCHOR: here
fn main() {
    let mut kullanici1 = Kullanici {
        aktif: true,
        kullanici_adi: String::from("birkullaniciadi123"),
        eposta: String::from("birisi@example.com"),
        giris_sayisi: 1,
    };

    kullanici1.eposta = String::from("baskaeposta@example.com");
}
// ANCHOR_END: here
