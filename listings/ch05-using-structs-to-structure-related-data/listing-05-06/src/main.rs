struct Kullanici {
    aktif: bool,
    kullanici_adi: String,
    eposta: String,
    giris_sayisi: u64,
}

// ANCHOR: here
fn main() {
    // --snip--
    // ANCHOR_END: here

    let kullanici1 = Kullanici {
        eposta: String::from("birisi@example.com"),
        kullanici_adi: String::from("birkullaniciadi123"),
        aktif: true,
        giris_sayisi: 1,
    };
    // ANCHOR: here

    let kullanici2 = Kullanici {
        aktif: kullanici1.aktif,
        kullanici_adi: kullanici1.kullanici_adi,
        eposta: String::from("baska@example.com"),
        giris_sayisi: kullanici1.giris_sayisi,
    };
}
// ANCHOR_END: here
