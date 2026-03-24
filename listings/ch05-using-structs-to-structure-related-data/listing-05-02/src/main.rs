struct Kullanici {
    aktif: bool,
    kullanici_adi: String,
    eposta: String,
    giris_sayisi: u64,
}

// ANCHOR: here
fn main() {
    let kullanici1 = Kullanici {
        aktif: true,
        kullanici_adi: String::from("birkullaniciadi123"),
        eposta: String::from("birisi@example.com"),
        giris_sayisi: 1,
    };
}
// ANCHOR_END: here
