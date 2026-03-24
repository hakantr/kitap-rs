struct Kullanici {
    aktif: bool,
    kullanici_adi: String,
    eposta: String,
    giris_sayisi: u64,
}

// ANCHOR: here
fn kullanici_olustur(eposta: String, kullanici_adi: String) -> Kullanici {
    Kullanici {
        aktif: true,
        kullanici_adi,
        eposta,
        giris_sayisi: 1,
    }
}
// ANCHOR_END: here

fn main() {
    let kullanici1 = kullanici_olustur(
        String::from("birisi@example.com"),
        String::from("birkullaniciadi123"),
    );
}
